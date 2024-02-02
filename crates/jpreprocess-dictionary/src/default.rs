use jpreprocess_core::{error::JPreprocessErrorKind, word_entry::WordEntry, JPreprocessResult};
use lindera_tokenizer::token::Token;

use super::{
    serializer::{jpreprocess::JPreprocessSerializer, lindera::LinderaSerializer},
    DictionaryFetcher, DictionarySerializer, DictionaryStore,
};

pub struct DefaultFetcher {
    pub system: WordDictionaryMode,
    pub user: Option<WordDictionaryMode>,
}

impl DefaultFetcher {
    pub fn new(system: WordDictionaryMode, user: Option<WordDictionaryMode>) -> Self {
        Self { system, user }
    }

    pub fn from_dictionaries<System, User>(system: &System, user: Option<&User>) -> Self
    where
        System: for<'a> DictionaryStore<'a>,
        User: for<'a> DictionaryStore<'a>,
    {
        Self {
            system: WordDictionaryMode::from_metadata(system.identifier()),
            user: user.map(|user| WordDictionaryMode::from_metadata(user.identifier())),
        }
    }
}

impl DictionaryFetcher for DefaultFetcher {
    fn get_word(&self, token: &Token) -> JPreprocessResult<WordEntry> {
        if token.word_id.is_unknown() {
            Ok(WordEntry::default())
        } else if token.word_id.is_system() {
            self.system
                .into_serializer()
                .deserialize(token.dictionary.get_bytes(token.word_id.0)?)
        } else if let Some(ref user_dict) = self.user {
            user_dict.into_serializer().deserialize(
                token
                    .user_dictionary
                    .ok_or(
                        JPreprocessErrorKind::WordNotFoundError.with_error(anyhow::anyhow!(
                "The word is flagged as UserDictionary, but Lindera UserDictionary is empty."
            )),
                    )?
                    .get_bytes(token.word_id.0)?,
            )
        } else {
            Err(
                JPreprocessErrorKind::WordNotFoundError.with_error(anyhow::anyhow!(
                    "The word is flagged as UserDictionary, but UserDictionary mode is not set."
                )),
            )
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum WordDictionaryMode {
    Lindera,
    JPreprocess,
    JPreprocessLegacyV051,
}

impl WordDictionaryMode {
    pub fn into_serializer(self) -> Box<dyn DictionarySerializer + Send + Sync> {
        match self {
            Self::Lindera => Box::new(LinderaSerializer),
            Self::JPreprocess => Box::new(JPreprocessSerializer),
            Self::JPreprocessLegacyV051 => {
                Box::new(crate::serializer::jpreprocess::legacy_0_5_1::JPreprocessSerializer)
            }
        }
    }

    pub fn from_metadata(metadata: Option<String>) -> Self {
        if let Some(metadata) = metadata {
            let segments: Vec<&str> = metadata.split(' ').collect();
            match *segments.as_slice() {
                ["JPreprocess", "v0.1.0" | "v0.1.1" | "v0.2.0"] => {
                    panic!(concat!(
                        "Incompatible Dictionary! ",
                        "Dictionaries built with JPreprocess versions before v0.3.0 ",
                        "are not compatible with this version of JPreprocess."
                    ))
                }
                ["JPreprocess", "v0.3.0" | "v0.4.0" | "v0.5.0" | "v0.5.1"] => {
                    return Self::JPreprocessLegacyV051
                }
                ["JPreprocess", ..] => return Self::JPreprocess,
                _ => (),
            }
        }
        Self::Lindera
    }
}
