use std::collections::BTreeMap;

use csv::StringRecord;
use daachorse::DoubleArrayAhoCorasickBuilder;
use lindera_dictionary::{
    error::LinderaErrorKind,
    viterbi::{LexType, WordEntry, WordId},
    LinderaResult,
};

use super::parser::CSVParser;

/// Build word entry map
pub fn build_word_entry_map<P: CSVParser>(
    parser: &P,
    rows: &[StringRecord],
    lex_type: LexType,
) -> LinderaResult<BTreeMap<String, Vec<WordEntry>>> {
    let mut word_entry_map: BTreeMap<String, Vec<WordEntry>> = BTreeMap::new();

    for (row_id, row) in rows.iter().enumerate() {
        let surface = parser.surface(row);
        let word_cost = parser.cost(row);
        let left_id = parser.left_context_id(row);
        let right_id = parser.right_context_id(row);

        // Skip if any value is invalid
        let (Ok(surface), Ok(word_cost), Ok(left_id), Ok(right_id)) =
            (surface, word_cost, left_id, right_id)
        else {
            continue;
        };

        word_entry_map
            .entry(surface)
            .or_default()
            .push(WordEntry::new(
                WordId::new(lex_type, row_id as u32),
                word_cost,
                left_id,
                right_id,
            ));
    }

    Ok(word_entry_map)
}

/// Generate values (dict.vals)
pub fn generate_values(
    word_entry_map: &BTreeMap<String, Vec<WordEntry>>,
) -> LinderaResult<Vec<u8>> {
    fn serialize_word_entry<W: std::io::Write>(
        entry: &WordEntry,
        wtr: &mut W,
    ) -> std::io::Result<()> {
        use byteorder::{LittleEndian, WriteBytesExt};

        // Copied from lindera_dictionary::viterbi::WordEntry::serialize
        wtr.write_u32::<LittleEndian>(entry.word_id().id())?;
        wtr.write_i16::<LittleEndian>(entry.word_cost())?;
        wtr.write_u16::<LittleEndian>(entry.left_id() as u16)?;
        wtr.write_u16::<LittleEndian>(entry.right_id() as u16)?;
        Ok(())
    }

    let mut dict_vals_buffer = Vec::new();
    for word_entries in word_entry_map.values() {
        for word_entry in word_entries {
            serialize_word_entry(word_entry, &mut dict_vals_buffer).map_err(|err| {
                LinderaErrorKind::Serialize
                    .with_error(anyhow::anyhow!(err))
                    .add_context(format!(
                        "Failed to serialize word entry (id: {})",
                        word_entry.word_id().id()
                    ))
            })?;
        }
    }

    Ok(dict_vals_buffer)
}

/// Generate double array for user prefix dictionary
pub fn generate_user_double_array(
    word_entry_map: &BTreeMap<String, Vec<WordEntry>>,
) -> LinderaResult<Vec<u8>> {
    let mut id = 0u32;
    let mut keyset: Vec<(&[u8], u32)> = vec![];
    for (key, word_entries) in word_entry_map {
        let len = word_entries.len() as u32;
        // 24bit for word ID, 8bit for variant count (up to 255 per surface),
        // matching the system dictionary encoding. The legacy 5-bit user
        // encoding (max 31 variants) was retired in v4.0.0; user dictionary
        // `.bin` files built with v3 must be rebuilt from their CSV source.
        let val = (id << 8) | len;
        keyset.push((key.as_bytes(), val));
        id += len;
    }

    Ok(DoubleArrayAhoCorasickBuilder::new()
        .build_with_values(keyset)
        .map_err(|err| {
            LinderaErrorKind::Build
                .with_error(anyhow::anyhow!(err))
                .add_context("Failed to build DoubleArray for user dictionary")
        })?
        .serialize())
}

#[cfg(test)]
mod tests {
    use crate::dictionary::to_dict::prefix_dictionary::parser::UserDictionaryParserOptions;

    use super::*;

    #[test]
    fn test_parse_simple_user_dictionary() {
        let data = vec![
            vec![
                "東京スカイツリー",       // surface
                "トウキョウスカイツリー", // reading
                "トーキョースカイツリー", // pronunciation
            ],
            vec![
                "すもももももももものうち",
                "スモモモモモモモモノウチ",
                "スモモモモモモモモノウチ",
            ],
        ];

        let csv_rows: Vec<StringRecord> = data.into_iter().map(StringRecord::from).collect();

        let parser = UserDictionaryParserOptions::default()
            .user_dictionary_fields_num(3)
            .default_word_cost(-10000)
            .default_left_context_id(0)
            .default_right_context_id(0)
            .builder()
            .unwrap();

        let word_entry_map = build_word_entry_map(&parser, &csv_rows, LexType::User).unwrap();
        assert_eq!(word_entry_map.len(), 2);
        assert_eq!(
            word_entry_map["東京スカイツリー"],
            vec![WordEntry::new(WordId::new(LexType::User, 0), -10000, 0, 0),]
        );
        assert_eq!(
            word_entry_map["すもももももももものうち"],
            vec![WordEntry::new(WordId::new(LexType::User, 1), -10000, 0, 0),]
        );
    }
}
