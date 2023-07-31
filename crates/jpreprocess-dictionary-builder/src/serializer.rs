use lindera_core::{error::LinderaErrorKind, LinderaResult};

pub trait DictionarySerializer {
    fn identifier(&self) -> String;
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>>;
    fn simple(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
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
    fn deserialize(&self, data: &[u8], string: String) -> LinderaResult<String>;
}

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
    fn deserialize(&self, data: &[u8], _string: String) -> LinderaResult<String> {
        bincode::deserialize(data)
            .map(|v: Vec<String>| v.join(","))
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(anyhow::anyhow!(err)))
    }
}

pub struct JPreprocessSerializer;
impl DictionarySerializer for JPreprocessSerializer {
    fn identifier(&self) -> String {
        format!("JPreprocess v{}", env!("CARGO_PKG_VERSION"))
    }
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        use jpreprocess_core::word_entry::WordEntry;
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
    fn deserialize(&self, data: &[u8], string: String) -> LinderaResult<String> {
        use jpreprocess_core::word_entry::WordEntry;
        let word_entry: WordEntry = bincode::deserialize(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(anyhow::anyhow!(err)))?;
        Ok(word_entry.to_str_vec(string).join(","))
    }
}
