//! This file is for compatibility with dictionaries compiled with
//! jpreprocess v0.7.0 and below.
//!
//! The serializer in this file cannot be used for building new dictionary,
//! because building dictionary with new core and old serializer
//! may result in jpreprocess-core structual incompatibility.
//! If you wish to build dictionary for older version,
//! please use the same older version of jpreprocess.
//!
//! To developers: Do not change the algorithm of the serializer in this file.

use bincode::Options;
use lindera_core::{error::LinderaErrorKind, LinderaResult};

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::{core_compat::v0_7_0, DictionarySerializer};

use self::bincode_serializer::SERIALIZE_OPTION;

mod bincode_serializer {
    use bincode::config::*;
    use once_cell::sync::Lazy;

    type Serializer = WithOtherTrailing<
        WithOtherIntEncoding<
            WithOtherEndian<WithOtherLimit<DefaultOptions, Infinite>, LittleEndian>,
            VarintEncoding,
        >,
        AllowTrailing,
    >;
    pub static SERIALIZE_OPTION: Lazy<Serializer> = Lazy::new(|| {
        bincode::config::DefaultOptions::new()
            .with_no_limit()
            .with_little_endian()
            .with_varint_encoding()
            .allow_trailing_bytes()
    });
}

pub struct JPreprocessSerializer;

impl DictionarySerializer for JPreprocessSerializer {
    fn identifier(&self) -> String {
        panic!("`legacy_0_7_0.rs` exists only for backward compatibility. Do not build dictionary with it.")
    }
    fn serialize(&self, _row: &[String]) -> LinderaResult<Vec<u8>> {
        panic!("`legacy_0_7_0.rs` exists only for backward compatibility. Do not build dictionary with it.")
    }

    fn deserialize(&self, data: &[u8]) -> JPreprocessResult<WordEntry> {
        let entry: v0_7_0::WordEntry = SERIALIZE_OPTION
            .deserialize(data)
            .map_err(DictionaryError::from)?;
        Ok(entry.into())
    }
    fn deserialize_debug(&self, data: &[u8]) -> String {
        format!("{:?}", self.deserialize(data))
    }
    fn deserialize_with_string(&self, data: &[u8], string: String) -> LinderaResult<String> {
        let word_entry: v0_7_0::WordEntry = SERIALIZE_OPTION
            .deserialize(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(anyhow::anyhow!(err)))?;
        Ok(WordEntry::from(word_entry).to_str_vec(string).join(","))
    }
}

#[cfg(test)]
mod tests {
    use crate::DictionarySerializer;

    use super::JPreprocessSerializer;

    const OKIBI: [u8; 33] = [
        0, 10, 2, 12, 27, 1, 9, 227, 130, 170, 227, 130, 173, 227, 131, 147, 3, 144, 1, 141, 1, 59,
        1, 0, 6, 1, 6, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn serialize() {
        let serlializer = JPreprocessSerializer;
        let input_str = "名詞,一般,*,*,*,*,おき火,オキビ,オキビ,0/3,C2,-1";

        let deserialized = serlializer
            .deserialize(&OKIBI.as_slice())
            .unwrap()
            .to_str_vec("おき火".to_string())
            .join(",");
        assert_eq!(input_str, deserialized);
    }
}
