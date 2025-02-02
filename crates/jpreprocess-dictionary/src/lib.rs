use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};
use lindera_tokenizer::token::Token;

pub mod default;

pub mod serializer;
pub mod store;

pub mod dictionary;

/// Fetch [`WordEntry`] corresponding to the given [`Token`].
pub trait DictionaryFetcher {
    fn get_word(&self, token: &Token) -> JPreprocessResult<WordEntry>;
    fn get_word_vectored(&self, tokens: &[Token]) -> JPreprocessResult<Vec<WordEntry>> {
        tokens.iter().map(|token| self.get_word(token)).collect()
    }
}
impl<T> DictionaryFetcher for T
where
    T: AsRef<dyn DictionaryFetcher>,
{
    fn get_word(&self, token: &Token) -> JPreprocessResult<WordEntry> {
        self.as_ref().get_word(token)
    }
    fn get_word_vectored(&self, tokens: &[Token]) -> JPreprocessResult<Vec<WordEntry>> {
        self.as_ref().get_word_vectored(tokens)
    }
}

/// Dictionary storage trait.
pub trait DictionaryStore<'a> {
    /// Get binary data for the word with the given id.
    fn get_bytes(&'a self, id: u32) -> JPreprocessResult<&'a [u8]>;
    /// Get the identifier (e.g. the variant or version of this dictionary).
    fn identifier(&self) -> Option<String>;
}
impl<'a, T> DictionaryStore<'a> for T
where
    T: AsRef<dyn DictionaryStore<'a>>,
{
    fn get_bytes(&'a self, id: u32) -> JPreprocessResult<&'a [u8]> {
        self.as_ref().get_bytes(id)
    }
    fn identifier(&self) -> Option<String> {
        self.as_ref().identifier()
    }
}
