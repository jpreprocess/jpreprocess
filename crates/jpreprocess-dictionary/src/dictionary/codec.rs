use lindera_dictionary::{error::LinderaErrorKind, LinderaResult};

/// A trait for encoding and decoding as dictionary entry.
pub trait DictionaryDataCodec: Sized {
    fn identifier() -> &'static str;
    fn encode(&self) -> LinderaResult<Vec<u8>>;
    fn decode(data: &[u8]) -> LinderaResult<Self>;
}

impl DictionaryDataCodec for Vec<String> {
    fn identifier() -> &'static str {
        concat!(
            "lindera [built with jpreprocess ",
            env!("CARGO_PKG_VERSION"),
            "]"
        )
    }

    fn encode(&self) -> LinderaResult<Vec<u8>> {
        bincode::serialize(self).map_err(|err| LinderaErrorKind::Serialize.with_error(err).into())
    }

    fn decode(data: &[u8]) -> LinderaResult<Self> {
        bincode::deserialize(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(err).into())
    }
}

impl DictionaryDataCodec for jpreprocess_core::word_entry::WordEntry {
    fn identifier() -> &'static str {
        concat!("jpreprocess ", env!("CARGO_PKG_VERSION"))
    }

    fn encode(&self) -> LinderaResult<Vec<u8>> {
        bincode::serialize(self).map_err(|err| LinderaErrorKind::Serialize.with_error(err).into())
    }

    fn decode(data: &[u8]) -> LinderaResult<Self> {
        bincode::deserialize(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(err).into())
    }
}

pub trait DictionaryRowCodec: Sized {
    fn encode(&self, string: String) -> LinderaResult<Vec<String>>;
    fn decode(row: &[&str]) -> LinderaResult<Self>;
}

impl DictionaryRowCodec for Vec<String> {
    fn encode(&self, _: String) -> LinderaResult<Vec<String>> {
        Ok(self.clone())
    }

    fn decode(row: &[&str]) -> LinderaResult<Self> {
        Ok(row.iter().map(|s| s.to_string()).collect())
    }
}

impl DictionaryRowCodec for jpreprocess_core::word_entry::WordEntry {
    fn encode(&self, string: String) -> LinderaResult<Vec<String>> {
        Ok(self.to_str_vec(string).to_vec())
    }

    fn decode(row: &[&str]) -> LinderaResult<Self> {
        jpreprocess_core::word_entry::WordEntry::load(row)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(err).into())
    }
}
