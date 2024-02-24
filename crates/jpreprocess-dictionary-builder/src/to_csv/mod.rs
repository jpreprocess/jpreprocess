use crate::build_dict::WordEntryMap;
use byteorder::{ByteOrder, LittleEndian};
use jpreprocess_dictionary::DictionarySerializer;
use lindera_core::{prefix_dict::PrefixDict, word_entry::WordEntry, LinderaResult};
use std::collections::BTreeMap;

use self::da::DoubleArrayParser;

mod da;

/// Converts dictionary to csv.
///
/// The third column (right_id) cannot be recovered
/// because it is already lost while building the dictionary.
pub fn dict_to_csv<S: DictionarySerializer>(
    prefix_dict: &PrefixDict,
    words_idx_data: &[u8],
    words_data: &[u8],
    serializer: &S,
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

    let words: Vec<String> = rows.iter().map(|w| w.0.to_owned()).collect();

    Ok(rows
        .into_iter()
        .zip(words_to_csv(words_idx_data, words_data, words, serializer)?)
        .map(|((string, word_entry), right)| {
            format!(
                "{},{},{},{},{}",
                string, word_entry.left_id, word_entry.right_id, word_entry.word_cost, right
            )
        })
        .collect())
}

/// Converts prefix dict to WordEntry map.
///
/// This is considered to be inverse of build_prefix_dict,
/// and no data is expected to be lost.
pub fn inverse_prefix_dict(prefix_dict: &PrefixDict, is_system: bool) -> WordEntryMap {
    let mut result: WordEntryMap = BTreeMap::new();

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

/// Converts words data to csv.
///
/// Note that some data (e.g. orig) is already lost in dictionary build process
/// if jpreprocess serlializer is used.
pub fn words_to_csv<S: DictionarySerializer>(
    words_idx_data: &[u8],
    words_data: &[u8],
    words: Vec<String>,
    serializer: &S,
) -> LinderaResult<Vec<String>> {
    let words_count = words_idx_data.len() / 4;
    assert_eq!(words_count, words.len());

    let mut result = vec![];
    for (i, word) in words.into_iter().enumerate() {
        let idx = LittleEndian::read_u32(&words_idx_data[i * 4..(i + 1) * 4]) as usize;
        let deserialized = serializer.deserialize_with_string(&words_data[idx..], word)?;
        result.push(deserialized);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use jpreprocess_dictionary::serializer::lindera::LinderaSerializer;

    use crate::ipadic_builder::IpadicBuilder;

    use super::dict_to_csv;

    #[test]
    fn inverse() -> Result<(), Box<dyn Error>> {
        let rows=[
            "キログラム,1360,1360,7944,名詞,接尾,助数詞,*,*,*,キログラム,キログラム,キログラム,3/5,C1,-1",
            "生麦生米生卵,3,3,10000,感動詞,*,*,*,*,*,生麦:生米:生卵,ナマムギ:ナマゴメ:ナマタマゴ,ナマムギ:ナマゴメ:ナマタマゴ,2/4:2/4:3/5,*,-1",
            "日本,1354,1354,10787,名詞,固有名詞,地域,国,*,*,日本,ニホン,ニホン,2/3,C1,-1"
        ];

        let rows_split: Vec<Vec<&str>> = rows.map(|s| s.split(',').collect()).to_vec();

        let builder = IpadicBuilder::new(Box::new(LinderaSerializer));
        let user_dict = builder.build_user_dict_from_data(&rows_split)?;

        let inverse = dict_to_csv(
            &user_dict.dict,
            &user_dict.words_idx_data,
            &user_dict.words_data,
            &LinderaSerializer,
        )?;

        assert_eq!(inverse[0], rows[0]);
        assert_eq!(inverse[1], rows[2]);
        assert_eq!(inverse[2], rows[1]);
        Ok(())
    }
}
