use std::borrow::Cow;

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::word_data::get_word_data;

use super::{PrefixDictionary, Token, Tokenizer};

pub struct JPreprocessTokenizer {
    tokenizer: lindera_tokenizer::tokenizer::Tokenizer,
}

impl JPreprocessTokenizer {
    pub fn new(tokenizer: lindera_tokenizer::tokenizer::Tokenizer) -> Self {
        Self { tokenizer }
    }

    fn get_word(
        &self,
        word_id: lindera_core::word_entry::WordId,
    ) -> Result<WordEntry, DictionaryError> {
        if word_id.is_unknown() {
            Ok(WordEntry::default())
        } else if word_id.is_system() {
            Self::get_word_from_prefixdict(
                &PrefixDictionary::from_dictionary(&self.tokenizer.dictionary),
                word_id,
            )
        } else {
            let user = &self.tokenizer.user_dictionary;
            user.as_ref()
                .map_or(Err(DictionaryError::UserDictionaryNotProvided), |user| {
                    Self::get_word_from_prefixdict(
                        &&PrefixDictionary::from_user_dictionary(user),
                        word_id,
                    )
                })
        }
    }

    /// PANIC: It must be ensured that the prefix_dict is the correct dictionary for the word_id.
    pub(super) fn get_word_from_prefixdict(
        prefix_dict: &PrefixDictionary,
        word_id: lindera_core::word_entry::WordId,
    ) -> Result<WordEntry, DictionaryError> {
        if word_id.is_unknown() {
            Ok(WordEntry::default())
        } else {
            let data = get_word_data(
                &prefix_dict.words_idx_data,
                &prefix_dict.words_data,
                Some(word_id.0 as usize),
            )
            .ok_or(DictionaryError::IdNotFound(word_id.0))?;
            Ok(bincode::deserialize(data)?)
        }
    }
}

impl Tokenizer for JPreprocessTokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        let words = self.tokenizer.tokenize(text).unwrap();
        words
            .into_iter()
            .map(|token| {
                Ok(JPreprocessToken::new(
                    token.text,
                    self.get_word(token.word_id)?,
                ))
            })
            .collect::<Result<_, _>>()
    }
}

pub struct JPreprocessToken<'a> {
    text: Cow<'a, str>,
    entry: WordEntry,
}

impl<'a> JPreprocessToken<'a> {
    pub(crate) fn new(text: &'a str, entry: WordEntry) -> Self {
        Self {
            text: Cow::Borrowed(text),
            entry,
        }
    }
}

impl Token for JPreprocessToken<'_> {
    fn fetch(&mut self) -> Result<(&str, WordEntry), jpreprocess_core::JPreprocessError> {
        Ok((&self.text, self.entry.clone()))
    }
}
