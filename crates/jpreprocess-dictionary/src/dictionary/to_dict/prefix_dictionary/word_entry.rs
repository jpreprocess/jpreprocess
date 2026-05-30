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

        word_entry_map.entry(surface).or_default().push(WordEntry {
            word_id: WordId::new(lex_type, row_id as u32),
            word_cost,
            left_id,
            right_id,
        });
    }

    Ok(word_entry_map)
}

/// Generate double array (dict.da)
pub fn generate_double_array(
    word_entry_map: &BTreeMap<String, Vec<WordEntry>>,
    is_system: bool,
) -> LinderaResult<Vec<u8>> {
    let mut id = 0u32;
    let mut keyset: Vec<(&[u8], u32)> = vec![];

    for (key, word_entries) in word_entry_map {
        let len = word_entries.len() as u32;

        // System dictionary: 24bit for word ID, 8bit for different parts of speech on the same surface.
        // User dictionary: 27bit for word ID, 5bit for different parts of speech on the same surface.
        let val = if is_system {
            (id << 8) | len
        } else {
            (id << 5) | len
        };

        keyset.push((key.as_bytes(), val));
        id += len;
    }

    let keyset_len = keyset.len();

    let dict_da = DoubleArrayAhoCorasickBuilder::new()
        .build_with_values(keyset)
        .map_err(|err| {
            LinderaErrorKind::Build
                .with_error(anyhow::anyhow!(err))
                .add_context(format!(
                    "Failed to build DoubleArray with {} keys for prefix dictionary",
                    keyset_len
                ))
        })?;

    let dict_da_buffer = dict_da.serialize();

    Ok(dict_da_buffer)
}

/// Generate values (dict.vals)
pub fn generate_values(
    word_entry_map: &BTreeMap<String, Vec<WordEntry>>,
) -> LinderaResult<Vec<u8>> {
    let mut dict_vals_buffer = Vec::new();
    for word_entries in word_entry_map.values() {
        for word_entry in word_entries {
            word_entry.serialize(&mut dict_vals_buffer).map_err(|err| {
                LinderaErrorKind::Serialize
                    .with_error(anyhow::anyhow!(err))
                    .add_context(format!(
                        "Failed to serialize word entry (id: {})",
                        word_entry.word_id.id
                    ))
            })?;
        }
    }

    Ok(dict_vals_buffer)
}
