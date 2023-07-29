use std::path::PathBuf;

use jpreprocess_core::{error::JPreprocessErrorKind, JPreprocessResult};
use jpreprocess_dictionary::WordDictionaryMode;
use lindera_core::mode::Mode;
use lindera_dictionary::{load_dictionary_from_config, DictionaryConfig};
use lindera_tokenizer::tokenizer::Tokenizer;

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
    pub(crate) fn load(self) -> JPreprocessResult<(Tokenizer, WordDictionaryMode)> {
        let dictionary = match self {
            Self::Bundled(kind) => kind.load(),
            Self::File(dictionary_path) => load_dictionary_from_config(DictionaryConfig {
                kind: None,
                path: Some(dictionary_path),
            })
            .map_err(|err| JPreprocessErrorKind::LinderaError.with_error(err))?,
        };

        let word_dict_config = detect_dictionary(&dictionary.words_data);
        let tokenizer = Tokenizer::new(dictionary, None, Mode::Normal);

        Ok((tokenizer, word_dict_config))
    }
}

pub fn detect_dictionary(words_data: &[u8]) -> WordDictionaryMode {
    if words_data.starts_with(b"JPreprocess") {
        WordDictionaryMode::JPreprocess
    } else {
        WordDictionaryMode::Lindera
    }
}
