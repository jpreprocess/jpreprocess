use lindera_core::{error::LinderaErrorKind, LinderaResult};

use jpreprocess_core::{error::JPreprocessErrorKind, word_entry::WordEntry, JPreprocessResult};

use super::DictionarySerializer;

pub struct JPreprocessSerializer;
impl DictionarySerializer for JPreprocessSerializer {
    fn identifier(&self) -> String {
        format!("JPreprocess v{}", env!("CARGO_PKG_VERSION"))
    }
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        let mut str_details = row.iter().map(|d| &d[..]).collect::<Vec<&str>>();
        str_details.resize(13, "");
        match WordEntry::load(&str_details[..]) {
            Ok(entry) => bincode::serialize(&entry)
                .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err))),
            Err(err) => {
                eprintln!("ERR: jpreprocess parse failed. Word:\n{:?}", &row);
                Err(LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))
            }
        }
    }

    fn deserialize(&self, data: &[u8]) -> JPreprocessResult<WordEntry> {
        let details: WordEntry = bincode::deserialize(data)
            .map_err(|err| JPreprocessErrorKind::WordNotFoundError.with_error(err))?;
        Ok(details)
    }
    fn deserialize_debug(&self, data: &[u8]) -> String {
        format!("{:?}", self.deserialize(data))
    }
    fn deserialize_with_string(&self, data: &[u8], string: String) -> JPreprocessResult<String> {
        let word_entry: WordEntry = bincode::deserialize(data).map_err(|err| {
            JPreprocessErrorKind::WordNotFoundError.with_error(anyhow::anyhow!(err))
        })?;
        Ok(word_entry.to_str_vec(string).join(","))
    }
}
