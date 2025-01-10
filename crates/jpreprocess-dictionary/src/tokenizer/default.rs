use std::borrow::Cow;

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry};

use super::{Token, Tokenizer};

fn get_word_data<'a>(idx: &[u8], data: &'a [u8], word_id: usize) -> &'a [u8] {
    if word_id * 4 + 4 > idx.len() {
        &[]
    } else if word_id * 4 + 8 > idx.len() {
        let start = u32::from_le_bytes([
            idx[word_id * 4],
            idx[word_id * 4 + 1],
            idx[word_id * 4 + 2],
            idx[word_id * 4 + 3],
        ]) as usize;
        &data[start..]
    } else {
        let start = u32::from_le_bytes([
            idx[word_id * 4],
            idx[word_id * 4 + 1],
            idx[word_id * 4 + 2],
            idx[word_id * 4 + 3],
        ]) as usize;
        let end = u32::from_le_bytes([
            idx[word_id * 4 + 4],
            idx[word_id * 4 + 5],
            idx[word_id * 4 + 6],
            idx[word_id * 4 + 7],
        ]) as usize;
        &data[start..end]
    }
}

pub struct DefaultTokenizer {
    tokenizer: lindera::tokenizer::Tokenizer,
}

impl DefaultTokenizer {
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
                word_id.id as usize,
            );
            Ok(bincode::deserialize(data)?)
        } else {
            let user = &self.tokenizer.segmenter.user_dictionary;
            user.as_ref()
                .map_or(Err(DictionaryError::UserDictionaryNotProvided), |user| {
                    let data = get_word_data(
                        &user.dict.words_idx_data,
                        &user.dict.words_data,
                        word_id.id as usize,
                    );
                    Ok(bincode::deserialize(data)?)
                })
        }
    }
}

impl Tokenizer for DefaultTokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> Vec<impl 'a + Token> {
        let words = self.tokenizer.tokenize(text).unwrap();
        words
            .into_iter()
            .map(|token| JPreprocessToken {
                text: token.text,
                entry: self.get_word(token.word_id).unwrap(),
            })
            .collect()
    }
}

pub struct JPreprocessToken<'a> {
    text: Cow<'a, str>,
    entry: WordEntry,
}

impl Token for JPreprocessToken<'_> {
    fn get_string(&mut self) -> &str {
        &self.text
    }
    fn get_word_entry(&mut self) -> WordEntry {
        self.entry.clone()
    }
}
