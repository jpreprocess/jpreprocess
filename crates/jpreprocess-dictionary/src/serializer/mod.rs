pub mod jpreprocess;
pub mod lindera;

use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};
use lindera_core::LinderaResult;

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
