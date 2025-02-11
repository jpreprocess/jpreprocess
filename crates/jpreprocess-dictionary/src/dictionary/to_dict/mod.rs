use std::{fs, path::Path};

use lindera_dictionary::{
    decompress::Algorithm,
    dictionary::{character_definition::CharacterDefinition, UserDictionary},
    dictionary_builder::{
        build_user_dictionary, CharacterDefinitionBuilderOptions,
        ConnectionCostMatrixBuilderOptions, DictionaryBuilder, UnknownDictionaryBuilderOptions,
    },
    error::LinderaErrorKind,
    LinderaResult,
};
use prefix_dictionary::PrefixDictionaryBuilderOptions;
use writer::{PrefixDictionaryDataWriter, PrefixDictionaryFileWriter};

use super::word_encoder::JPreprocessDictionaryWordEncoder;

mod prefix_dictionary;
mod writer;

const SIMPLE_USERDIC_FIELDS_NUM: usize = 3;
const SIMPLE_WORD_COST: i16 = -10000;
const SIMPLE_CONTEXT_ID: u16 = 0;
// const DETAILED_USERDIC_FIELDS_NUM: usize = 13;
const COMPRESS_ALGORITHM: Algorithm = Algorithm::Raw;
const UNK_FIELDS_NUM: usize = 11;

pub struct JPreprocessDictionaryBuilder {}

impl JPreprocessDictionaryBuilder {
    pub fn new() -> Self {
        JPreprocessDictionaryBuilder {}
    }
}

impl DictionaryBuilder for JPreprocessDictionaryBuilder {
    fn build_dictionary(&self, input_dir: &Path, output_dir: &Path) -> LinderaResult<()> {
        fs::create_dir_all(output_dir)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        let chardef = self.build_character_definition(input_dir, output_dir)?;
        self.build_unknown_dictionary(input_dir, &chardef, output_dir)?;
        self.build_prefix_dictionary(input_dir, output_dir)?;
        self.build_connection_cost_matrix(input_dir, output_dir)?;

        Ok(())
    }

    fn build_user_dictionary(&self, input_file: &Path, output_file: &Path) -> LinderaResult<()> {
        let user_dict = self.build_user_dict(input_file)?;
        build_user_dictionary(user_dict, output_file)
    }

    fn build_character_definition(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<CharacterDefinition> {
        CharacterDefinitionBuilderOptions::default()
            .compress_algorithm(COMPRESS_ALGORITHM)
            .builder()
            .unwrap()
            .build(input_dir, output_dir)
    }

    fn build_unknown_dictionary(
        &self,
        input_dir: &Path,
        chardef: &CharacterDefinition,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        UnknownDictionaryBuilderOptions::default()
            .compress_algorithm(COMPRESS_ALGORITHM)
            .unk_fields_num(UNK_FIELDS_NUM)
            .builder()
            .unwrap()
            .build(input_dir, chardef, output_dir)
    }

    fn build_prefix_dictionary(&self, input_dir: &Path, output_dir: &Path) -> LinderaResult<()> {
        let pattern = if let Some(path) = input_dir.to_str() {
            format!("{}/*.csv", path)
        } else {
            return Err(
                LinderaErrorKind::Io.with_error(anyhow::anyhow!("Failed to convert path to &str."))
            );
        };

        let mut filenames = Vec::new();
        for entry in glob::glob(&pattern)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?
        {
            match entry {
                Ok(path) => {
                    if let Some(filename) = path.file_name() {
                        filenames.push(Path::new(input_dir).join(filename));
                    } else {
                        return Err(LinderaErrorKind::Io
                            .with_error(anyhow::anyhow!("failed to get filename")));
                    };
                }
                Err(err) => return Err(LinderaErrorKind::Content.with_error(anyhow::anyhow!(err))),
            }
        }

        let mut rows = vec![];
        for filename in filenames {
            log::debug!("reading {:?}", filename);

            let file = std::fs::File::open(filename)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;
            let reader = Box::new(file);
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(false)
                .from_reader(reader);

            for result in rdr.records() {
                let record = result
                    .map_err(|err| LinderaErrorKind::Content.with_error(anyhow::anyhow!(err)))?;
                rows.push(record);
            }
        }

        let mut writer = PrefixDictionaryFileWriter::new(output_dir);

        PrefixDictionaryBuilderOptions::default()
            .normalize_details(true)
            .builder()
            .unwrap()
            .build::<JPreprocessDictionaryWordEncoder, _>(rows, &mut writer)
    }

    fn build_connection_cost_matrix(
        &self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> LinderaResult<()> {
        ConnectionCostMatrixBuilderOptions::default()
            .compress_algorithm(COMPRESS_ALGORITHM)
            .builder()
            .unwrap()
            .build(input_dir, output_dir)
    }

    fn build_user_dict(&self, input_file: &Path) -> LinderaResult<UserDictionary> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(input_file)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        let mut rows = vec![];
        for result in rdr.records() {
            let record =
                result.map_err(|err| LinderaErrorKind::Content.with_error(anyhow::anyhow!(err)))?;
            rows.push(record);
        }

        let mut writer = PrefixDictionaryDataWriter::new();

        PrefixDictionaryBuilderOptions::default()
            .is_user_dict(true)
            .simple_userdic_fields_num(SIMPLE_USERDIC_FIELDS_NUM)
            .simple_word_cost(SIMPLE_WORD_COST)
            .simple_context_id(SIMPLE_CONTEXT_ID)
            .builder()
            .unwrap()
            .build::<JPreprocessDictionaryWordEncoder, _>(rows, &mut writer)?;

        Ok(UserDictionary {
            dict: writer.build_prefix_dictionary(false),
        })
    }
}

pub fn build_user_dict_from_data(data: Vec<Vec<&str>>) -> LinderaResult<UserDictionary> {
    let data = data.into_iter().map(csv::StringRecord::from_iter).collect();

    let mut writer = PrefixDictionaryDataWriter::new();

    PrefixDictionaryBuilderOptions::default()
        .is_user_dict(true)
        .simple_userdic_fields_num(SIMPLE_USERDIC_FIELDS_NUM)
        .simple_word_cost(SIMPLE_WORD_COST)
        .simple_context_id(SIMPLE_CONTEXT_ID)
        .builder()
        .unwrap()
        .build::<JPreprocessDictionaryWordEncoder, _>(data, &mut writer)?;

    Ok(UserDictionary {
        dict: writer.build_prefix_dictionary(false),
    })
}
