use byteorder::{ByteOrder, LittleEndian};
use lindera_dictionary::{error::LinderaErrorKind, LinderaResult};

/// A trait for encoding and decoding as dictionary entry.
pub trait DictionaryWordEncoder: Sized {
    fn identifier() -> &'static str;
    fn encode(row: &[&str]) -> LinderaResult<Vec<u8>>;
    fn decode(string: String, details: &[u8]) -> LinderaResult<Vec<String>>;
}

pub struct JPreprocessDictionaryWordEncoder;
impl DictionaryWordEncoder for JPreprocessDictionaryWordEncoder {
    fn identifier() -> &'static str {
        concat!("jpreprocess", env!("CARGO_PKG_VERSION"))
    }

    fn encode(row: &[&str]) -> LinderaResult<Vec<u8>> {
        let mut row = row.to_vec();
        row.resize(13, "");
        let data = jpreprocess_core::word_entry::WordEntry::load(&row)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(err).into())?;
        bincode::serialize(&data).map_err(|err| LinderaErrorKind::Serialize.with_error(err).into())
    }

    fn decode(string: String, data: &[u8]) -> LinderaResult<Vec<String>> {
        let word_details: jpreprocess_core::word_entry::WordEntry = bincode::deserialize(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(err))?;
        Ok(word_details.to_str_vec(string).to_vec())
    }
}

pub struct LinderaSystemDictionaryWordEncoder;
impl DictionaryWordEncoder for LinderaSystemDictionaryWordEncoder {
    fn identifier() -> &'static str {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn encode(_row: &[&str]) -> LinderaResult<Vec<u8>> {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn decode(_string: String, data: &[u8]) -> LinderaResult<Vec<String>> {
        let len = LittleEndian::read_u32(data) as usize;
        let data = &data[4..4+len];

        let mut details = Vec::new();
        for bytes in data.split(|&b| b == 0) {
            let detail = match std::str::from_utf8(bytes) {
                Ok(s) => s,
                Err(err) => return Err(LinderaErrorKind::Deserialize.with_error(err)),
            };
            details.push(detail.to_string());
        }
        Ok(details)
    }
}

pub struct LinderaUserDictionaryWordEncoder;
impl DictionaryWordEncoder for LinderaUserDictionaryWordEncoder {
    fn identifier() -> &'static str {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn encode(_row: &[&str]) -> LinderaResult<Vec<u8>> {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn decode(_string: String, data: &[u8]) -> LinderaResult<Vec<String>> {
        bincode::deserialize(data).map_err(|err| LinderaErrorKind::Deserialize.with_error(err))
    }
}
