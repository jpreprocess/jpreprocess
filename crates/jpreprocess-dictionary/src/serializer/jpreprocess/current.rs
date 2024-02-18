use bincode::Options;
use lindera_core::{error::LinderaErrorKind, LinderaResult};

use jpreprocess_core::{error::DictionaryError, word_entry::WordEntry, JPreprocessResult};

use crate::DictionarySerializer;

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
        format!("JPreprocess v{}", env!("CARGO_PKG_VERSION"))
    }
    fn serialize(&self, row: &[String]) -> LinderaResult<Vec<u8>> {
        let mut str_details = row.iter().map(|d| &d[..]).collect::<Vec<&str>>();
        str_details.resize(13, "");
        match WordEntry::load(&str_details[..]) {
            Ok(entry) => SERIALIZE_OPTION
                .serialize(&entry)
                .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err))),
            Err(err) => {
                eprintln!("ERR: jpreprocess parse failed. Word:\n{:?}", &row);
                Err(LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))
            }
        }
    }

    fn deserialize(&self, data: &[u8]) -> JPreprocessResult<WordEntry> {
        let details: WordEntry = SERIALIZE_OPTION
            .deserialize(data)
            .map_err(DictionaryError::from)?;
        Ok(details)
    }
    fn deserialize_debug(&self, data: &[u8]) -> String {
        format!("{:?}", self.deserialize(data))
    }
    fn deserialize_with_string(&self, data: &[u8], string: String) -> LinderaResult<String> {
        let word_entry: WordEntry = SERIALIZE_OPTION
            .deserialize(data)
            .map_err(|err| LinderaErrorKind::Deserialize.with_error(anyhow::anyhow!(err)))?;
        Ok(word_entry.to_str_vec(string).join(","))
    }
}

#[cfg(test)]
mod tests {
    use crate::DictionarySerializer;

    use super::JPreprocessSerializer;

    const OKIBI: [u8; 32] = [
        0, 10, 2, 12, 27, 1, 9, 227, 130, 170, 227, 130, 173, 227, 131, 147, 3, 144, 1, 141, 1, 59,
        1, 0, 1, 6, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn serialize() {
        let serlializer = JPreprocessSerializer;
        let input_str = "名詞,一般,*,*,*,*,おき火,オキビ,オキビ,0/3,C2,-1";
        let input: Vec<String> = input_str.split(',').map(str::to_string).collect();
        let bytes = serlializer.serialize(&input).unwrap();
        assert_eq!(&bytes, OKIBI.as_slice());

        let deserialized = serlializer
            .deserialize(&bytes)
            .unwrap()
            .to_str_vec("おき火".to_string())
            .join(",");
        assert_eq!(input_str, deserialized);
    }
}
