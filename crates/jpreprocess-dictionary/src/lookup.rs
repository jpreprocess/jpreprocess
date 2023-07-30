use byteorder::{ByteOrder, LittleEndian};
use jpreprocess_core::{error::JPreprocessErrorKind, word_entry::WordEntry};
use lindera_tokenizer::token::Token;

use jpreprocess_core::JPreprocessResult;

use crate::query::DictionaryQuery;

#[derive(Clone, Copy, Debug)]
pub struct WordDictionaryConfig {
    pub system: WordDictionaryMode,
    pub user: Option<WordDictionaryMode>,
}

impl WordDictionaryConfig {
    pub fn get_word(&self, token: &Token) -> JPreprocessResult<WordEntry> {
        if token.word_id.is_unknown() {
            Ok(WordEntry::default())
        } else if token.word_id.is_system() {
            self.system.get_word(token)
        } else if let Some(user_dict) = self.user {
            user_dict.get_word(token)
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
    pub fn get_word(&self, query: &(dyn DictionaryQuery)) -> JPreprocessResult<WordEntry> {
        let details_bin = Self::get_word_binary(query)?;
        match self {
            Self::Lindera => {
                let mut details_str: Vec<&str> = bincode::deserialize(details_bin)
                    .map_err(|err| JPreprocessErrorKind::WordNotFoundError.with_error(err))?;
                details_str.resize(13, "");
                WordEntry::load(&details_str)
            }
            Self::JPreprocess => {
                let details: WordEntry = bincode::deserialize(details_bin)
                    .map_err(|err| JPreprocessErrorKind::WordNotFoundError.with_error(err))?;
                Ok(details)
            }
        }
    }

    pub fn debug_get_word(&self, query: &(dyn DictionaryQuery)) -> String {
        let details_bin = match Self::get_word_binary(query) {
            Ok(details_bin) => details_bin,
            Err(err) => return format!("Error: {:?}", err),
        };
        match self {
            Self::Lindera => match bincode::deserialize::<'_, Vec<&str>>(details_bin) {
                Ok(details_str) => details_str.join(","),
                Err(err) => format!("Error: {:?}", err),
            },
            Self::JPreprocess => match bincode::deserialize::<'_, WordEntry>(details_bin) {
                Ok(details) => format!("{:?}", details),
                Err(err) => format!("Error: {:?}", err),
            },
        }
    }

    fn get_word_binary<'a>(query: &'a (dyn DictionaryQuery)) -> JPreprocessResult<&'a [u8]> {
        let (words_idx_data, words_data) = if query.word_id().is_system() {
            (
                &query.dictionary().words_idx_data[..],
                &query.dictionary().words_data[..],
            )
        } else {
            match query.user_dictionary() {
                Some(user_dictionary) => (
                    &user_dictionary.words_idx_data[..],
                    &user_dictionary.words_data[..],
                ),
                None => {
                    return Err(JPreprocessErrorKind::WordNotFoundError.with_error(
                        anyhow::anyhow!(
                    "The word is flagged as UserDictionary, but Lindera UserDictionary is empty."
                ),
                    ))
                }
            }
        };

        let start_point = 4 * query.word_id().0 as usize;
        if words_idx_data.len() < start_point + 4 {
            return Err(
                JPreprocessErrorKind::WordNotFoundError.with_error(anyhow::anyhow!(
                    "Word with id {:?} does not exist.",
                    query.word_id()
                )),
            );
        }

        let idx = LittleEndian::read_u32(&words_idx_data[start_point..start_point + 4]) as usize;
        let idx_next = if words_idx_data.len() < start_point + 8 {
            words_data.len()
        } else {
            LittleEndian::read_u32(&words_idx_data[start_point + 4..start_point + 8]) as usize
        };

        Ok(&words_data[idx..idx_next])
    }
}
