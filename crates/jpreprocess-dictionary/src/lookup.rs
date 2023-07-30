use byteorder::{ByteOrder, LittleEndian};
use jpreprocess_core::{error::JPreprocessErrorKind, word_entry::WordEntry};
use lindera_tokenizer::token::Token;

use jpreprocess_core::JPreprocessResult;

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
    pub fn get_word(&self, token: &Token) -> JPreprocessResult<WordEntry> {
        let details_bin = Self::get_word_binary(token)?;
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

    fn get_word_binary<'a>(token: &'a Token) -> JPreprocessResult<&'a [u8]> {
        let (words_idx_data, words_data) = if token.word_id.is_system() {
            (
                &token.dictionary.words_idx_data[..],
                &token.dictionary.words_data[..],
            )
        } else {
            match token.user_dictionary {
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

        let start_point = 4 * token.word_id.0 as usize;
        if words_idx_data.len() < start_point + 4 {
            return Err(
                JPreprocessErrorKind::WordNotFoundError.with_error(anyhow::anyhow!(
                    "Word index {:?} is out of range.",
                    token.word_id
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
