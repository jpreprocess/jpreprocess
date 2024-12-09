use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
};

use lindera::error::LinderaError;
use lindera_dictionary::{
    dictionary::prefix_dictionary::PrefixDictionary, error::LinderaErrorKind,
};
use yada::DoubleArray;

pub enum PrefixDictionaryDataType {
    DoubleArray,
    Vals,
    Words,
    WordsIdx,
}

pub trait PrefixDictionaryWriter {
    fn write(
        &mut self,
        dict_type: PrefixDictionaryDataType,
        data: &[u8],
    ) -> Result<(), LinderaError>;
}

pub struct PrefixDictionaryFileWriter {
    output_dir: PathBuf,
}

impl PrefixDictionaryFileWriter {
    pub fn new(output_dir: &Path) -> Self {
        PrefixDictionaryFileWriter {
            output_dir: output_dir.to_path_buf(),
        }
    }
}

impl PrefixDictionaryWriter for PrefixDictionaryFileWriter {
    fn write(
        &mut self,
        dict_type: PrefixDictionaryDataType,
        data: &[u8],
    ) -> Result<(), LinderaError> {
        let file_path = match dict_type {
            PrefixDictionaryDataType::DoubleArray => self.output_dir.join("double_array.bin"),
            PrefixDictionaryDataType::Vals => self.output_dir.join("vals.bin"),
            PrefixDictionaryDataType::Words => self.output_dir.join("words.bin"),
            PrefixDictionaryDataType::WordsIdx => self.output_dir.join("words_idx.bin"),
        };

        let mut wtr = io::BufWriter::new(
            File::create(file_path)
                .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?,
        );

        wtr.write_all(data)
            .map_err(|err| LinderaErrorKind::Io.with_error(anyhow::anyhow!(err)))?;

        Ok(())
    }
}

pub struct PrefixDictionaryDataWriter {
    da: Option<DoubleArray<Vec<u8>>>,
    vals_data: Option<Vec<u8>>,
    words_idx_data: Option<Vec<u8>>,
    words_data: Option<Vec<u8>>,
}

impl PrefixDictionaryDataWriter {
    fn new() -> Self {
        PrefixDictionaryDataWriter {
            da: None,
            vals_data: None,
            words_idx_data: None,
            words_data: None,
        }
    }
    fn build_prefix_dictionary(&self, is_system: bool) -> PrefixDictionary {
        PrefixDictionary {
            da: self.da.clone().unwrap(),
            vals_data: self.vals_data.clone().unwrap(),
            words_idx_data: self.words_idx_data.clone().unwrap(),
            words_data: self.words_data.clone().unwrap(),
            is_system,
        }
    }
}

impl PrefixDictionaryWriter for PrefixDictionaryDataWriter {
    fn write(
        &mut self,
        dict_type: PrefixDictionaryDataType,
        data: &[u8],
    ) -> Result<(), LinderaError> {
        match dict_type {
            PrefixDictionaryDataType::DoubleArray => {
                self.da = Some(DoubleArray::new(data.to_vec()));
            }
            PrefixDictionaryDataType::Vals => {
                self.vals_data = Some(data.to_vec());
            }
            PrefixDictionaryDataType::Words => {
                self.words_data = Some(data.to_vec());
            }
            PrefixDictionaryDataType::WordsIdx => {
                self.words_idx_data = Some(data.to_vec());
            }
        }

        Ok(())
    }
}
