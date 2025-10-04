use lindera_dictionary::{
    dictionary::prefix_dictionary::PrefixDictionary, viterbi::WordEntry, LinderaResult,
};
use std::collections::BTreeMap;

use crate::word_data::get_word_data;

use self::da::DoubleArrayParser;

use super::{word_encoding::DictionaryWordEncoding, WordEntryMap};

mod da;

/// Converts prefix dictionary back to csv.
pub fn dict_to_csv<E: DictionaryWordEncoding>(
    prefix_dictionary: &PrefixDictionary,
) -> LinderaResult<Vec<String>> {
    let word_entry_map = inverse_prefix_dict(prefix_dictionary, true);

    let rows: Vec<(String, WordEntry)> = word_entry_map
        .into_iter()
        .flat_map(|(string, word_entries)| {
            word_entries
                .into_iter()
                .map(move |word_entry| (string.to_owned(), word_entry))
        })
        .collect();

    Ok(rows
        .into_iter()
        .map(|(string, word_entry)| {
            let word_data = get_word_data(
                &prefix_dictionary.words_idx_data,
                &prefix_dictionary.words_data,
                Some(word_entry.word_id.id as usize),
            )
            .unwrap();
            let details = E::decode(string.clone(), word_data).unwrap();

            format!(
                "{},{},{},{},{}",
                string,
                word_entry.left_id,
                word_entry.right_id,
                word_entry.word_cost,
                details.join(",")
            )
        })
        .collect())
}

/// Converts prefix dict to WordEntry map.
///
/// This is considered to be inverse of build_prefix_dict,
/// and no data is expected to be lost.
pub fn inverse_prefix_dict(prefix_dictionary: &PrefixDictionary, is_system: bool) -> WordEntryMap {
    let mut result: WordEntryMap = BTreeMap::new();

    let keyset = DoubleArrayParser(&prefix_dictionary.da.0).inverse_da();
    for (s, offset_len) in keyset {
        let len = offset_len & 0x1f;
        let offset = offset_len >> 5;
        let offset_bytes = (offset as usize) * WordEntry::SERIALIZED_LEN;
        let data: &[u8] = &prefix_dictionary.vals_data[offset_bytes..];
        result.insert(
            s,
            (0..len as usize)
                .map(move |i| {
                    WordEntry::deserialize(&data[WordEntry::SERIALIZED_LEN * i..], is_system)
                })
                .collect(),
        );
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::dictionary::word_encoding::{
        JPreprocessDictionaryWordEncoding, LinderaUserDictionaryWordEncoding,
    };

    use super::dict_to_csv;
    use std::{error::Error, path::PathBuf};

    #[test]
    fn inverse_lindera() -> Result<(), Box<dyn Error>> {
        let input_file = PathBuf::from("./test.csv");

        let builder =
            lindera_dictionary::builder::user_dictionary::UserDictionaryBuilderOptions::default()
                .builder()
                .unwrap();
        let user_dict = builder.build(&input_file).unwrap();

        let inverse = dict_to_csv::<LinderaUserDictionaryWordEncoding>(&user_dict.dict)?;

        let input_content = std::fs::read_to_string(input_file).unwrap();
        let rows = input_content.split('\n').collect::<Vec<_>>();

        assert_eq!(inverse[0], rows[0]);
        assert_eq!(inverse[1], rows[2]);
        assert_eq!(inverse[2], rows[1]);
        Ok(())
    }

    #[test]
    fn inverse_jpreprocess() -> Result<(), Box<dyn Error>> {
        let input_file = PathBuf::from("./test.csv");

        let builder = crate::dictionary::to_dict::JPreprocessDictionaryBuilder::new();
        let user_dict = builder.build_user_dict(&input_file).unwrap();

        let inverse = dict_to_csv::<JPreprocessDictionaryWordEncoding>(&user_dict.dict)?;

        let input_content = std::fs::read_to_string(input_file).unwrap();
        let rows = input_content.split('\n').collect::<Vec<_>>();

        assert_eq!(inverse[0], rows[0]);
        assert_eq!(inverse[1], rows[2]);
        assert_eq!(inverse[2], rows[1]);
        Ok(())
    }
}
