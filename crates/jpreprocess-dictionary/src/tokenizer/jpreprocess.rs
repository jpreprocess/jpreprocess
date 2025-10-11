use std::borrow::Cow;

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::{
    dictionary::word_encoding::JPreprocessDictionaryWordEncoding, word_data::get_word_data,
};

use super::{Token, Tokenizer};

pub struct JPreprocessTokenizer {
    tokenizer: lindera::tokenizer::Tokenizer,
}

impl JPreprocessTokenizer {
    pub fn new(tokenizer: lindera::tokenizer::Tokenizer) -> Self {
        Self { tokenizer }
    }

    fn get_word(
        &self,
        word_id: lindera_dictionary::viterbi::WordId,
    ) -> Result<WordEntry, DictionaryError> {
        if word_id.is_unknown() {
            Ok(WordEntry::default())
        } else if word_id.is_system() {
            Self::get_word_from_prefixdict(
                &self.tokenizer.segmenter.dictionary.prefix_dictionary,
                word_id,
            )
        } else {
            let user = &self.tokenizer.segmenter.user_dictionary;
            user.as_ref()
                .map_or(Err(DictionaryError::UserDictionaryNotProvided), |user| {
                    Self::get_word_from_prefixdict(&user.dict, word_id)
                })
        }
    }

    /// PANIC: It must be ensured that the prefix_dict is the correct dictionary for the word_id.
    pub(super) fn get_word_from_prefixdict(
        prefix_dict: &lindera_dictionary::dictionary::prefix_dictionary::PrefixDictionary,
        word_id: lindera_dictionary::viterbi::WordId,
    ) -> Result<WordEntry, DictionaryError> {
        if word_id.is_unknown() {
            Ok(WordEntry::default())
        } else {
            let data = get_word_data(
                &prefix_dict.words_idx_data,
                &prefix_dict.words_data,
                Some(word_id.id as usize),
            )
            .ok_or(DictionaryError::IdNotFound(word_id.id))?;
            Ok(JPreprocessDictionaryWordEncoding::deserialize(data)?)
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
                    token.surface,
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
    pub(crate) fn new(text: Cow<'a, str>, entry: WordEntry) -> Self {
        Self { text, entry }
    }
}

impl Token for JPreprocessToken<'_> {
    fn fetch(&mut self) -> Result<(&str, WordEntry), jpreprocess_core::JPreprocessError> {
        Ok((&self.text, self.entry.clone()))
    }
}
