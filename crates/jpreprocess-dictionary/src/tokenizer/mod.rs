use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::{
    serializer::{lindera::LinderaSerializer, DictionarySerializer},
    word_data::get_word_data,
};

pub mod default;
mod identify_dictionary;
pub mod jpreprocess;

struct PrefixDictionary<'a> {
    words_idx_data: &'a [u8],
    words_data: &'a [u8],
}

impl<'a> PrefixDictionary<'a> {
    fn from_dictionary(dictionary: &'a lindera_core::dictionary::Dictionary) -> Self {
        Self {
            words_idx_data: &dictionary.words_idx_data,
            words_data: &dictionary.words_data,
        }
    }
    fn from_user_dictionary(user_dictionary: &'a lindera_core::dictionary::UserDictionary) -> Self {
        Self {
            words_idx_data: &user_dictionary.words_idx_data,
            words_data: &user_dictionary.words_data,
        }
    }
}

pub trait Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>>;
}

pub trait Token {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)>;
}

impl Tokenizer for lindera_tokenizer::tokenizer::Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        Ok(self.tokenize(text)?)
    }
}

impl Token for lindera_tokenizer::token::Token<'_> {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)> {
        // FIXME: Rewrite this to the following when lindera is updated:
        // let mut details = self.details();
        // let entry = if details == *UNK {
        //     WordEntry::default()
        // } else {
        //     details.resize(13, "");
        //     WordEntry::load(&details)?
        // };

        let entry = if self.word_id.is_unknown() {
            WordEntry::default()
        } else if self.word_id.is_system() {
            let word_data = get_word_data(
                &self.dictionary.words_idx_data,
                &self.dictionary.words_data,
                Some(self.word_id.0 as usize),
            )
            .ok_or(DictionaryError::IdNotFound(self.word_id.0))?;

            LinderaSerializer {}.deserialize(word_data)?
        } else if let Some(user_dictionary) = self.user_dictionary {
            let word_data = get_word_data(
                &user_dictionary.words_idx_data,
                &user_dictionary.words_data,
                Some(self.word_id.0 as usize),
            )
            .ok_or(DictionaryError::IdNotFound(self.word_id.0))?;

            LinderaSerializer {}.deserialize(word_data)?
        } else {
            return Err(DictionaryError::UserDictionaryNotProvided.into());
        };

        Ok((self.text, entry))
    }
}
