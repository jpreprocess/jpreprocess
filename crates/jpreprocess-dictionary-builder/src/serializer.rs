use lindera_core::{error::LinderaErrorKind, LinderaResult};

pub trait DictionarySerializer {
    fn identifier(&self) -> String;
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>>;
}

pub struct LinderaSerializer;
impl DictionarySerializer for LinderaSerializer {
    fn identifier(&self) -> String {
        "Lindera".to_string()
    }
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        let mut word_detail = Vec::new();
        for item in row.iter().skip(4) {
            word_detail.push(item.to_string());
        }
        bincode::serialize(&word_detail)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))
    }
}

pub struct JPreprocessSerializer;
impl DictionarySerializer for JPreprocessSerializer {
    fn identifier(&self) -> String {
        format!("JPreprocess v{}", env!("CARGO_PKG_VERSION"))
    }
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        use jpreprocess_core::word_entry::WordEntry;
        let mut str_details = row.iter().skip(4).map(|d| &d[..]).collect::<Vec<&str>>();
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
}
