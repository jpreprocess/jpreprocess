use jpreprocess_core::word_line::WordDetailsLine;
use lindera_dictionary::{error::LinderaErrorKind, LinderaResult};

/// A trait for encoding and decoding as dictionary entry.
pub trait DictionaryWordEncoding: Sized {
    fn identifier() -> &'static str;
    fn encode(row: WordDetailsLine) -> LinderaResult<Vec<u8>>;
}

pub struct JPreprocessDictionaryWordEncoding;
impl JPreprocessDictionaryWordEncoding {
    pub fn serialize(
        data: &jpreprocess_core::word_entry::WordEntry,
    ) -> Result<Vec<u8>, jpreprocess_core::error::DictionaryError> {
        Ok(data.to_bin())
    }
    pub fn deserialize(
        data: &[u8],
    ) -> Result<jpreprocess_core::word_entry::WordEntry, jpreprocess_core::error::DictionaryError>
    {
        let mut iter = data.iter().copied();
        let entry = jpreprocess_core::word_entry::WordEntry::from_bin(&mut iter)?;
        Ok(entry)
    }
}
impl DictionaryWordEncoding for JPreprocessDictionaryWordEncoding {
    fn identifier() -> &'static str {
        concat!("jpreprocess ", env!("CARGO_PKG_VERSION"))
    }

    fn encode(row: WordDetailsLine) -> LinderaResult<Vec<u8>> {
        let data = row
            .try_into()
            .map_err(|err| LinderaErrorKind::Serialize.with_error(err))?;
        Self::serialize(&data).map_err(|err| LinderaErrorKind::Serialize.with_error(err))
    }
}
