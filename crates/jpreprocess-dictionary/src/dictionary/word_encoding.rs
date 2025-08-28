use byteorder::{ByteOrder, LittleEndian};
use lindera_dictionary::{error::LinderaErrorKind, LinderaResult};

/// A trait for encoding and decoding as dictionary entry.
pub trait DictionaryWordEncoding: Sized {
    fn identifier() -> &'static str;
    fn encode(row: &[&str]) -> LinderaResult<Vec<u8>>;
    fn decode(string: String, details: &[u8]) -> LinderaResult<Vec<String>>;
}

pub struct JPreprocessDictionaryWordEncoding;
impl JPreprocessDictionaryWordEncoding {
    pub fn serialize(data: &jpreprocess_core::word_entry::WordEntry) -> bincode::Result<Vec<u8>> {
        use bincode::Options;
        Self::bincode_option().serialize(data)
    }
    pub fn deserialize(data: &[u8]) -> bincode::Result<jpreprocess_core::word_entry::WordEntry> {
        use bincode::Options;
        Self::bincode_option().deserialize(data)
    }

    fn bincode_option() -> impl bincode::Options {
        use bincode::Options;

        bincode::config::DefaultOptions::new()
            .with_no_limit()
            .with_little_endian()
            .with_varint_encoding()
            .allow_trailing_bytes()
    }
}
impl DictionaryWordEncoding for JPreprocessDictionaryWordEncoding {
    fn identifier() -> &'static str {
        concat!("jpreprocess ", env!("CARGO_PKG_VERSION"))
    }

    fn encode(row: &[&str]) -> LinderaResult<Vec<u8>> {
        let mut row = row.to_vec();
        row.resize(13, "");
        let data = jpreprocess_core::word_entry::WordEntry::load(&row)
            .map_err(|err| LinderaErrorKind::Serialize.with_error(err))?;
        Self::serialize(&data).map_err(|err| LinderaErrorKind::Serialize.with_error(err))
    }

    fn decode(string: String, data: &[u8]) -> LinderaResult<Vec<String>> {
        let word_details: jpreprocess_core::word_entry::WordEntry =
            Self::deserialize(data).map_err(|err| LinderaErrorKind::Deserialize.with_error(err))?;
        Ok(word_details.to_str_vec(string).to_vec())
    }
}

pub struct LinderaSystemDictionaryWordEncoding;
impl DictionaryWordEncoding for LinderaSystemDictionaryWordEncoding {
    fn identifier() -> &'static str {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn encode(_row: &[&str]) -> LinderaResult<Vec<u8>> {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn decode(_string: String, data: &[u8]) -> LinderaResult<Vec<String>> {
        let len = LittleEndian::read_u32(data) as usize;
        let data = &data[4..4 + len];

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

pub struct LinderaUserDictionaryWordEncoding;
impl DictionaryWordEncoding for LinderaUserDictionaryWordEncoding {
    fn identifier() -> &'static str {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn encode(_row: &[&str]) -> LinderaResult<Vec<u8>> {
        unimplemented!("JPreprocess does not support building in Lindera dictionary format")
    }

    fn decode(_string: String, data: &[u8]) -> LinderaResult<Vec<String>> {
        LinderaSystemDictionaryWordEncoding::decode(_string, data)
    }
}
