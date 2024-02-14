use lindera_core::{error::LinderaErrorKind, LinderaResult};

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::DictionarySerializer;

pub struct LinderaSerializer;
impl DictionarySerializer for LinderaSerializer {
    fn identifier(&self) -> String {
        "Lindera".to_string()
    }
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        let mut word_detail = Vec::new();
        for item in row.iter() {
            word_detail.push(item.to_string());
        }
        bincode::serialize(&word_detail)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))
    }

    fn deserialize(&self, data: &[u8]) -> JPreprocessResult<WordEntry> {
        let mut details_str: Vec<&str> =
            bincode::deserialize(data).map_err(DictionaryError::from)?;
        details_str.resize(13, "");
        WordEntry::load(&details_str)
    }
    fn deserialize_debug(&self, data: &[u8]) -> String {
        match bincode::deserialize::<'_, Vec<&str>>(data) {
            Ok(details_str) => details_str.join(","),
            Err(err) => format!("Error: {:?}", err),
        }
    }
    fn deserialize_with_string(&self, data: &[u8], _string: String) -> LinderaResult<String> {
        bincode::deserialize(data)
            .map(|v: Vec<String>| v.join(","))
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(anyhow::anyhow!(err)))
    }
}
