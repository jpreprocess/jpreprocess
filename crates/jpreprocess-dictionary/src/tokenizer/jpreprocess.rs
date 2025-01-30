use std::borrow::Cow;

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::word_data::get_word_data;

use super::{Token, Tokenizer};

pub struct JPreprocessTokenizer {
    tokenizer: lindera::tokenizer::Tokenizer,
}

impl JPreprocessTokenizer {
    pub fn new(tokenizer: lindera::tokenizer::Tokenizer) -> Self {
        Self { tokenizer }
    }

    fn get_word(&self, word_id: lindera::dictionary::WordId) -> Result<WordEntry, DictionaryError> {
        if word_id.is_unknown() {
            Ok(WordEntry::default())
        } else if word_id.is_system() {
            let system = &self.tokenizer.segmenter.dictionary.prefix_dictionary;
            let data = get_word_data(
                &system.words_idx_data,
                &system.words_data,
                Some(word_id.id as usize),
            )
            .ok_or(DictionaryError::IdNotFound(word_id.id))?;
            Ok(bincode::deserialize(data)?)
        } else {
            let user = &self.tokenizer.segmenter.user_dictionary;
            user.as_ref()
                .map_or(Err(DictionaryError::UserDictionaryNotProvided), |user| {
                    let data = get_word_data(
                        &user.dict.words_idx_data,
                        &user.dict.words_data,
                        Some(word_id.id as usize),
                    )
                    .ok_or(DictionaryError::IdNotFound(word_id.id))?;
                    Ok(bincode::deserialize(data)?)
                })
        }
    }
}

impl Tokenizer for JPreprocessTokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        let words = self.tokenizer.tokenize(text).unwrap();
        words
            .into_iter()
            .map(|token| {
                Ok(JPreprocessToken {
                    text: token.text,
                    entry: self.get_word(token.word_id)?,
                })
            })
            .collect::<Result<_, _>>()
    }
}

pub struct JPreprocessToken<'a> {
    text: Cow<'a, str>,
    entry: WordEntry,
}

impl Token for JPreprocessToken<'_> {
    fn fetch(&mut self) -> Result<(&str, WordEntry), jpreprocess_core::JPreprocessError> {
        Ok((&self.text, self.entry.clone()))
    }
}
