use byteorder::{ByteOrder, LittleEndian};
use lindera::LinderaResult;
use lindera_dictionary::{dictionary::prefix_dictionary::PrefixDictionary, viterbi::WordEntry};
use std::collections::BTreeMap;

use self::da::DoubleArrayParser;

mod da;

/// Converts dictionary to csv.
///
/// The third column (right_id) cannot be recovered
/// because it is lost while building the dictionary.
///
/// TODO: implement reverse jpreprocess dict
pub fn dict_to_csv(
    prefix_dict: &PrefixDictionary,
    words_idx_data: &[u8],
    words_data: &[u8],
) -> LinderaResult<Vec<String>> {
    let word_entry_map = inverse_prefix_dict(prefix_dict, true);

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
        .enumerate()
        .map(|(i, (string, word_entry))| {
            let idx = LittleEndian::read_u32(&words_idx_data[i * 4..(i + 1) * 4]) as usize;
            let details: Vec<String> = bincode::deserialize(&words_data[idx..]).unwrap();

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
pub fn inverse_prefix_dict(
    prefix_dict: &PrefixDictionary,
    is_system: bool,
) -> BTreeMap<String, Vec<WordEntry>> {
    let mut result = BTreeMap::new();

    let keyset = DoubleArrayParser(&prefix_dict.da.0).inverse_da();
    for (s, offset_len) in keyset {
        let len = offset_len & 0x1f;
        let offset = offset_len >> 5;
        let offset_bytes = (offset as usize) * WordEntry::SERIALIZED_LEN;
        let data: &[u8] = &prefix_dict.vals_data[offset_bytes..];
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
    use lindera_dictionary::dictionary_builder::DictionaryBuilder;

    use super::dict_to_csv;
    use std::{error::Error, path::PathBuf};

    #[test]
    fn inverse() -> Result<(), Box<dyn Error>> {
        let input_file = PathBuf::from("./test.csv");

        let builder = lindera_dictionary::dictionary_builder::ipadic::IpadicBuilder::new();
        let user_dict = builder.build_user_dict(&input_file).unwrap();

        let inverse = dict_to_csv(
            &user_dict.dict,
            &user_dict.dict.words_idx_data,
            &user_dict.dict.words_data,
        )?;

        let input_content = std::fs::read_to_string(input_file).unwrap();
        let rows = input_content.split('\n').collect::<Vec<_>>();

        assert_eq!(inverse[0], rows[0]);
        assert_eq!(inverse[1], rows[2]);
        assert_eq!(inverse[2], rows[1]);
        Ok(())
    }
}
