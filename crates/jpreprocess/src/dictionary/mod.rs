use std::path::PathBuf;

use jpreprocess_core::JPreprocessResult;
use lindera_core::dictionary::Dictionary;
use lindera_dictionary::{load_dictionary_from_config, DictionaryConfig};

pub mod kind;

/// System dictionary configuration for JPreprocess.
pub enum SystemDictionaryConfig {
    /// Use self-contained dictionary. This is only valid if appropreate feature is enabled.
    Bundled(kind::JPreprocessDictionaryKind),
    /// Use pre-built external lindera/jpreprocess dictionary. The PathBuf is the path to dictionary.
    ///
    /// - When you are using lindera dictionary: Normal dictionary cannot be used;
    ///   it must contain the accent position and accent rule.
    /// - When you are using jpreprocess dictionary: The JPreprocess version needs to be same as the
    ///   JPreprocess that built the dictionary.
    File(PathBuf),
}

impl SystemDictionaryConfig {
    pub fn load(self) -> JPreprocessResult<Dictionary> {
        let dictionary = match self {
            Self::Bundled(kind) => kind.load(),
            Self::File(dictionary_path) => load_dictionary_from_config(DictionaryConfig {
                kind: None,
                path: Some(dictionary_path),
            })?,
        };

        Ok(dictionary)
    }
}
