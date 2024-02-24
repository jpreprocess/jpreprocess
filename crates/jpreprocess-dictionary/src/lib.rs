use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};
use lindera_core::LinderaResult;
use lindera_tokenizer::token::Token;

pub mod default;

pub mod serializer;
pub mod store;

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

pub trait DictionarySerializer {
    // For dictionary builder
    fn identifier(&self) -> String;
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>>;
    fn serialize_simple(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        let details = vec![
            row[1].to_string(), // POS
            "*".to_string(),    // POS subcategory 1
            "*".to_string(),    // POS subcategory 2
            "*".to_string(),    // POS subcategory 3
            "*".to_string(),    // Conjugation type
            "*".to_string(),    // Conjugation form
            row[0].to_string(), // Base form
            row[2].to_string(), // Reading
            "*".to_string(),    // Pronunciation
        ];
        self.serialize(&details)
    }

    // For dictionary parser
    fn deserialize(&self, data: &[u8]) -> JPreprocessResult<WordEntry>;
    fn deserialize_debug(&self, data: &[u8]) -> String;

    // For dictionary restorer
    fn deserialize_with_string(&self, data: &[u8], string: String) -> LinderaResult<String>;
}
impl<T> DictionarySerializer for Box<T>
where
    T: DictionarySerializer + ?Sized,
{
    fn identifier(&self) -> String {
        self.as_ref().identifier()
    }
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        self.as_ref().serialize(row)
    }
    fn serialize_simple(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        self.as_ref().serialize_simple(row)
    }
    fn deserialize(&self, data: &[u8]) -> JPreprocessResult<WordEntry> {
        self.as_ref().deserialize(data)
    }
    fn deserialize_debug(&self, data: &[u8]) -> String {
        self.as_ref().deserialize_debug(data)
    }
    fn deserialize_with_string(&self, data: &[u8], string: String) -> LinderaResult<String> {
        self.as_ref().deserialize_with_string(data, string)
    }
}
