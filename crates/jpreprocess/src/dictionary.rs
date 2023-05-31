use std::path::PathBuf;

use jpreprocess_core::{error::JPreprocessErrorKind, JPreprocessResult};
use jpreprocess_dictionary::{DictionaryTrait, JPreprocessDictionary};
use lindera_core::mode::Mode;
use lindera_dictionary::DictionaryConfig;
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};

pub enum JPreprocessDictionaryKind {
    #[cfg(feature = "naist-jdic")]
    NaistJdic,
}

impl JPreprocessDictionaryKind {
    pub(crate) fn load(&self) -> (lindera_core::dictionary::Dictionary, JPreprocessDictionary) {
        match &self {
            #[cfg(feature = "naist-jdic")]
            Self::NaistJdic => (
                jpreprocess_naist_jdic::lindera::load_dictionary().unwrap(),
                jpreprocess_naist_jdic::jpreprocess::load_dictionary(),
            ),
            _ => unreachable!(),
        }
    }
}

pub enum JPreprocessDictionaryConfig {
    Bundled(JPreprocessDictionaryKind),
    FileLindera(PathBuf),
    FileJPreprocess(PathBuf),
}

impl JPreprocessDictionaryConfig {
    pub(crate) fn load(self) -> JPreprocessResult<(Tokenizer, Option<JPreprocessDictionary>)> {
        match self {
            Self::Bundled(kind) => {
                let (lindera_dictionary, jpreprocess_dictionary) = kind.load();
                let tokenizer = Tokenizer::new(lindera_dictionary, None, Mode::Normal);
                Ok((tokenizer, Some(jpreprocess_dictionary)))
            }
            Self::FileLindera(dictionary_path) => {
                let tokenizer = Self::lindera_tokenizer(dictionary_path)?;
                Ok((tokenizer, None))
            }
            Self::FileJPreprocess(dictionary_path) => {
                let tokenizer = Self::lindera_tokenizer(dictionary_path.clone())?;
                let jpreprocess_dictionary = JPreprocessDictionary::load(dictionary_path)?;
                Ok((tokenizer, Some(jpreprocess_dictionary)))
            }
        }
    }

    fn lindera_tokenizer(dictionary_path: PathBuf) -> JPreprocessResult<Tokenizer> {
        let dictionary = DictionaryConfig {
            kind: None,
            path: Some(dictionary_path),
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };
        Tokenizer::from_config(config)
            .map_err(|err| JPreprocessErrorKind::LinderaError.with_error(err))
    }
}
