//! This file is for compatibility with dictionaries compiled with
//! jpreprocess v0.5.1 and below.
//!
//! The serializer in this file cannot be used for building new dictionary,
//! because building dictionary with new core and old serializer
//! may result in jpreprocess-core structual incompatibility.
//! If you wish to build dictionary for older version,
//! please use the same older version of jpreprocess.
//!
//! To developers: Do not change the algorithm of the serializer in this file.

use lindera_core::{error::LinderaErrorKind, LinderaResult};

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::{core_compat::v0_7_0, DictionarySerializer};

pub struct JPreprocessSerializer;
impl DictionarySerializer for JPreprocessSerializer {
    fn identifier(&self) -> String {
        panic!("`legacy_0_5_1.rs` exists only for backward compatibility. Do not build dictionary with it.")
    }
    fn serialize(&self, _row: &[String]) -> LinderaResult<Vec<u8>> {
        panic!("`legacy_0_5_1.rs` exists only for backward compatibility. Do not build dictionary with it.")
    }

    fn deserialize(&self, data: &[u8]) -> JPreprocessResult<WordEntry> {
        let details: v0_7_0::WordEntry =
            bincode::deserialize(data).map_err(DictionaryError::from)?;
        Ok(details.into())
    }
    fn deserialize_debug(&self, data: &[u8]) -> String {
        format!("{:?}", self.deserialize(data))
    }
    fn deserialize_with_string(&self, data: &[u8], string: String) -> LinderaResult<String> {
        let word_entry: v0_7_0::WordEntry = bincode::deserialize(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(anyhow::anyhow!(err)))?;
        Ok(WordEntry::from(word_entry).to_str_vec(string).join(","))
    }
}

#[cfg(test)]
mod tests {
    use crate::DictionarySerializer;

    use super::JPreprocessSerializer;

    const OKIBI: [u8; 83] = [
        0, 0, 0, 0, 10, 0, 0, 0, 2, 0, 0, 0, 12, 0, 0, 0, 27, 0, 0, 0, 1, 9, 0, 0, 0, 0, 0, 0, 0,
        227, 130, 170, 227, 130, 173, 227, 131, 147, 3, 0, 0, 0, 0, 0, 0, 0, 144, 0, 0, 0, 1, 141,
        0, 0, 0, 1, 59, 0, 0, 0, 1, 0, 0, 0, 0, 3, 0, 0, 0, 1, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0,
    ];

    #[test]
    fn serialize() {
        let serlializer = JPreprocessSerializer;
        let input_str = "名詞,一般,*,*,*,*,おき火,オキビ,オキビ,0/3,C2,-1";

        let deserialized = serlializer
            .deserialize(&OKIBI)
            .unwrap()
            .to_str_vec("おき火".to_string())
            .join(",");
        assert_eq!(input_str, deserialized);
    }
}
