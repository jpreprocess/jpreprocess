use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    str::FromStr,
    u32,
};

use jpreprocess_dictionary::{serializer::lindera::LinderaSerializer, DictionarySerializer};
use rayon::prelude::*;

use byteorder::{LittleEndian, WriteBytesExt};
use csv::StringRecord;
use glob::glob;
use log::debug;

use lindera_core::{
    character_definition::{CharacterDefinitions, CharacterDefinitionsBuilder},
    dictionary::UserDictionary,
    dictionary_builder::DictionaryBuilder,
    error::LinderaErrorKind,
    file_util::read_utf8_file,
    unknown_dictionary::parse_unk,
    LinderaResult,
};

use crate::build_dict::*;

pub struct IpadicBuilder {
    serializer: Box<dyn DictionarySerializer + Send + Sync>,
}

impl IpadicBuilder {
    const UNK_FIELDS_NUM: usize = 11;

    pub fn new(serializer: Box<dyn Send + Sync + DictionarySerializer>) -> Self {
        IpadicBuilder { serializer }
    }

    fn write_words(
        &self,
        wtr_words_path: &Path,
        wtr_words_idx_path: &Path,
        is_system: bool,
        normalized_rows: &Vec<Vec<String>>,
    ) -> Result<(), lindera_core::error::LinderaError> {
        let mut wtr_words = io::BufWriter::new(
            File::create(wtr_words_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );
        let mut wtr_words_idx = io::BufWriter::new(
            File::create(wtr_words_idx_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );

        let (words_idx_buffer, words_buffer) =
            build_words(&self.serializer, normalized_rows, is_system)?;

        write(&words_buffer, &mut wtr_words)?;
        write(&words_idx_buffer, &mut wtr_words_idx)?;
        wtr_words
            .flush()
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;
        wtr_words_idx
            .flush()
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;
        Ok(())
    }

    pub fn build_user_dict_from_data(
        &self,
        rows: &Vec<Vec<&str>>,
    ) -> LinderaResult<UserDictionary> {
        let mut normalized_rows: Vec<Vec<String>> = normalize_rows(rows);
        normalized_rows.par_sort_by_key(|row| row.first().map(|s| s.to_string()));
        let (words_idx_data, words_data) = build_words(&self.serializer, &normalized_rows, false)?;
        let dict = build_prefix_dict(build_word_entry_map(&normalized_rows, false)?, false)?;
        Ok(UserDictionary {
            dict,
            words_idx_data,
            words_data,
        })
    }
}

impl Default for IpadicBuilder {
    fn default() -> Self {
        Self::new(Box::new(LinderaSerializer))
    }
}

impl DictionaryBuilder for IpadicBuilder {
    fn build_dictionary(&self, input_dir: &Path, output_dir: &Path) -> LinderaResult<()> {
        fs::create_dir_all(output_dir)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        let chardef = self.build_chardef(input_dir, output_dir)?;
        self.build_unk(input_dir, &chardef, output_dir)?;
        self.build_dict(input_dir, output_dir)?;
        self.build_cost_matrix(input_dir, output_dir)?;

        Ok(())
    }

    fn build_user_dictionary(&self, input_file: &Path, output_file: &Path) -> LinderaResult<()> {
        let parent_dir = match output_file.parent() {
            Some(parent_dir) => parent_dir,
            None => {
                return Err(LinderaErrorKind::Io.with_error(anyhow::anyhow!(
                    "failed to get parent directory of output file"
                )))
            }
        };
        fs::create_dir_all(parent_dir)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        let user_dict = self.build_user_dict(input_file)?;

        let mut wtr = io::BufWriter::new(
            File::create(output_file)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );
        bincode::serialize_into(&mut wtr, &user_dict)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;
        wtr.flush()
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        Ok(())
    }

    fn build_chardef(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<CharacterDefinitions> {
        let char_def_path = input_dir.join("char.def");
        debug!("reading {:?}", char_def_path);

        let char_def = read_utf8_file(&char_def_path)?;
        let mut char_definitions_builder = CharacterDefinitionsBuilder::default();
        char_definitions_builder.parse(&char_def)?;
        let char_definitions = char_definitions_builder.build();

        let mut chardef_buffer = Vec::new();
        bincode::serialize_into(&mut chardef_buffer, &char_definitions)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;

        let wtr_chardef_path = output_dir.join(Path::new("char_def.bin"));
        let mut wtr_chardef = io::BufWriter::new(
            File::create(wtr_chardef_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );

        write(&chardef_buffer, &mut wtr_chardef)?;

        wtr_chardef
            .flush()
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        Ok(char_definitions)
    }

    fn build_unk(
        &self,
        input_dir: &Path,
        chardef: &CharacterDefinitions,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        let unk_data_path = input_dir.join("unk.def");
        debug!("reading {:?}", unk_data_path);

        let unk_data = read_utf8_file(&unk_data_path)?;
        let unknown_dictionary = parse_unk(chardef.categories(), &unk_data, Self::UNK_FIELDS_NUM)?;

        let mut unk_buffer = Vec::new();
        bincode::serialize_into(&mut unk_buffer, &unknown_dictionary)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;

        let wtr_unk_path = output_dir.join(Path::new("unk.bin"));
        let mut wtr_unk = io::BufWriter::new(
            File::create(wtr_unk_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );
        write(&unk_buffer, &mut wtr_unk)?;
        wtr_unk
            .flush()
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        Ok(())
    }

    fn build_dict(&self, input_dir: &Path, output_dir: &Path) -> LinderaResult<()> {
        let pattern = if let Some(path) = input_dir.to_str() {
            format!("{}/*.csv", path)
        } else {
            return Err(
                LinderaErrorKind::Io.with_error(anyhow::anyhow!("Failed to convert path to &str."))
            );
        };

        let mut filenames: Vec<PathBuf> = Vec::new();
        for entry in
            glob(&pattern).map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?
        {
            match entry {
                Ok(path) => {
                    if let Some(filename) = path.file_name() {
                        filenames.push(Path::new(input_dir).join(filename));
                    } else {
                        return Err(LinderaErrorKind::Io
                            .with_error(anyhow::anyhow!("failed to get filename")));
                    }
                }
                Err(err) => return Err(LinderaErrorKind::Content.with_error(anyhow::anyhow!(err))),
            }
        }

        let mut rows: Vec<StringRecord> = vec![];
        for filename in filenames {
            debug!("reading {:?}", filename);

            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .from_path(filename)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

            for result in rdr.records() {
                let record = result
                    .map_err(|err| LinderaErrorKind::Content.with_error(anyhow::anyhow!(err)))?;
                rows.push(record);
            }
        }

        let mut normalized_rows: Vec<Vec<String>> = normalize_rows(&rows);

        normalized_rows.par_sort_by_key(|row| row.first().map(|s| s.to_string()));

        let wtr_da_path = output_dir.join(Path::new("dict.da"));
        let mut wtr_da = io::BufWriter::new(
            File::create(wtr_da_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );

        let wtr_vals_path = output_dir.join(Path::new("dict.vals"));
        let mut wtr_vals = io::BufWriter::new(
            File::create(wtr_vals_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );

        self.write_words(
            output_dir.join(Path::new("dict.words")).as_path(),
            output_dir.join(Path::new("dict.wordsidx")).as_path(),
            true,
            &normalized_rows,
        )?;

        let prefix_dict = build_prefix_dict(build_word_entry_map(&normalized_rows, true)?, true)?;

        write(&prefix_dict.da.0, &mut wtr_da)?;

        write(&prefix_dict.vals_data, &mut wtr_vals)?;

        wtr_vals
            .flush()
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        Ok(())
    }

    fn build_cost_matrix(&self, input_dir: &Path, output_dir: &Path) -> LinderaResult<()> {
        let matrix_data_path = input_dir.join("matrix.def");
        debug!("reading {:?}", matrix_data_path);

        let matrix_data = read_utf8_file(&matrix_data_path)?;
        let mut lines_it = matrix_data
            .par_lines()
            .map(|line| {
                line.split_whitespace()
                    .map(i32::from_str)
                    .collect::<Result<Vec<i32>, _>>()
                    .map_err(|err| LinderaErrorKind::Parse.with_error(anyhow::anyhow!(err)))
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter();
        let header = lines_it.next().ok_or_else(|| {
            LinderaErrorKind::Content.with_error(anyhow::anyhow!("unknown error"))
        })?;
        let forward_size = header[0] as u32;
        let backward_size = header[1] as u32;
        let len = 2 + (forward_size * backward_size) as usize;
        let mut costs = vec![i16::MAX; len];
        costs[0] = forward_size as i16;
        costs[1] = backward_size as i16;
        for fields in lines_it {
            let forward_id = fields[0] as u32;
            let backward_id = fields[1] as u32;
            let cost = fields[2] as u16;
            costs[2 + (backward_id + forward_id * backward_size) as usize] = cost as i16;
        }

        let wtr_matrix_mtx_path = output_dir.join(Path::new("matrix.mtx"));
        let mut wtr_matrix_mtx = io::BufWriter::new(
            File::create(wtr_matrix_mtx_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );
        let mut matrix_mtx_buffer = Vec::new();
        for cost in costs {
            matrix_mtx_buffer
                .write_i16::<LittleEndian>(cost)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;
        }
        write(&matrix_mtx_buffer, &mut wtr_matrix_mtx)?;

        wtr_matrix_mtx
            .flush()
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        Ok(())
    }

    fn build_user_dict(&self, input_file: &Path) -> LinderaResult<UserDictionary> {
        debug!("reading {:?}", input_file);

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(input_file)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        let mut rows: Vec<StringRecord> = vec![];
        for result in rdr.records() {
            let record =
                result.map_err(|err| LinderaErrorKind::Content.with_error(anyhow::anyhow!(err)))?;
            rows.push(record);
        }

        let mut normalized_rows: Vec<Vec<String>> = normalize_rows(&rows);
        normalized_rows.par_sort_by_key(|row| row.first().map(|s| s.to_string()));
        let (words_idx_data, words_data) = build_words(&self.serializer, &normalized_rows, false)?;
        let dict = build_prefix_dict(build_word_entry_map(&normalized_rows, false)?, false)?;

        Ok(UserDictionary {
            dict,
            words_idx_data,
            words_data,
        })
    }
}

fn write<W: Write>(buffer: &[u8], writer: &mut W) -> LinderaResult<()> {
    writer
        .write_all(buffer)
        .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

    Ok(())
}
