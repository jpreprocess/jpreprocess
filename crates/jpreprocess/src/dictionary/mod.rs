use std::path::PathBuf;

use jpreprocess_core::{error::JPreprocessErrorKind, JPreprocessResult};
use jpreprocess_dictionary::{DictionaryTrait, JPreprocessDictionary};
use lindera_core::mode::Mode;
use lindera_dictionary::DictionaryConfig;
use lindera_tokenizer::tokenizer::{Tokenizer, TokenizerConfig};

pub mod kind;

/// Dictionary configuration for JPreprocess.
/// 
/// The only difference between FileLindera and FileJPreprocess is how the words are stored in memory.
/// JPreprocess dictionary pre-parse the strings, and it consumes less memory,
/// whereas Lindera dictionary contains all the data in string.
pub enum JPreprocessDictionaryConfig {
    /// Use self-contained dictionary. This is only valid if appropreate feature is enabled.
    Bundled(kind::JPreprocessDictionaryKind),
    /// Use pre-built external lindera dictionary. The PathBuf is the path to dictionary.
    /// Please note that normal dictionary cannot be used; it must contain the accent position
    /// and accent rule.
    FileLindera(PathBuf),
    /// Use pre-built external jpreprocess dictionary. The PathBuf is the path to dictionary.
    /// Please note that the version of the dictionary must match the jpreprocess version you use.
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
