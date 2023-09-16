use jpreprocess_core::{error::JPreprocessErrorKind, word_entry::WordEntry, JPreprocessResult};
use lindera_tokenizer::token::Token;

use super::{
    serializer::{jpreprocess::JPreprocessSerializer, lindera::LinderaSerializer},
    DictionaryFetcher, DictionarySerializer, DictionaryStore,
};

pub struct WordDictionaryConfig {
    pub system: Box<dyn DictionarySerializer>,
    pub user: Option<Box<dyn DictionarySerializer>>,
}

impl WordDictionaryConfig {
    pub fn new(system: WordDictionaryMode, user: Option<WordDictionaryMode>) -> Self {
        Self {
            system: system.into_serializer(),
            user: user.map(WordDictionaryMode::into_serializer),
        }
    }
}

impl DictionaryFetcher for WordDictionaryConfig {
    fn get_word(&self, token: &Token) -> JPreprocessResult<WordEntry> {
        if token.word_id.is_unknown() {
            Ok(WordEntry::default())
        } else if token.word_id.is_system() {
            self.system
                .deserialize(token.dictionary.get_bytes(token.word_id.0)?)
        } else if let Some(ref user_dict) = self.user {
            user_dict.deserialize(
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
}

impl WordDictionaryMode {
    fn into_serializer(self) -> Box<dyn DictionarySerializer> {
        match self {
            Self::Lindera => Box::new(LinderaSerializer),
            Self::JPreprocess => Box::new(JPreprocessSerializer),
        }
    }
}
