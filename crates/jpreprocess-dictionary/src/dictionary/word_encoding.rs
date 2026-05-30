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
    ) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::serde::encode_to_vec(data, Self::bincode_option())
    }
    pub fn deserialize(
        data: &[u8],
    ) -> Result<jpreprocess_core::word_entry::WordEntry, bincode::error::DecodeError> {
        let (decoded, _size) = bincode::serde::decode_from_slice(data, Self::bincode_option())?;
        Ok(decoded)
    }

    fn bincode_option() -> bincode::config::Configuration {
        bincode::config::standard()
            .with_no_limit()
            .with_little_endian()
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
