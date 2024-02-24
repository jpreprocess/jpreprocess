use byteorder::{LittleEndian, WriteBytesExt};
use lindera_core::{
    error::LinderaErrorKind,
    prefix_dict::PrefixDict,
    word_entry::{WordEntry, WordId},
    LinderaResult,
};
use rayon::prelude::*;
use std::{collections::BTreeMap, str::FromStr};
use yada::{builder::DoubleArrayBuilder, DoubleArray};

use jpreprocess_dictionary::DictionarySerializer;

const SIMPLE_USERDIC_FIELDS_NUM: usize = 3;
const DETAILED_USERDIC_FIELDS_NUM: usize = 13;

const SIMPLE_WORD_COST: i16 = -10000;
const SIMPLE_CONTEXT_ID: u16 = 0;

pub type WordEntryMap = BTreeMap<String, Vec<WordEntry>>;

pub fn normalize_rows<'a, T, U, V>(rows: &'a T) -> Vec<Vec<String>>
where
    T: IntoParallelRefIterator<'a, Item = U>,
    U: 'a + IntoIterator<Item = &'a V>,
    V: 'a + ToString + ?Sized,
{
    rows.par_iter()
        .map(|row| {
            row.into_iter()
                // yeah for EUC_JP and ambiguous unicode 8012 vs 8013
                // same bullshit as above between for 12316 vs 65374
                .map(|column| column.to_string().replace('―', "—").replace('～', "〜"))
                .collect()
        })
        .collect()
}

pub fn build_word_entry_map(
    rows: &Vec<Vec<String>>,
    is_system: bool,
) -> LinderaResult<WordEntryMap> {
    let entries = rows
        .par_iter()
        .enumerate()
        .map(|(row_id, row)| {
            let is_simple = !is_system && row.len() == SIMPLE_USERDIC_FIELDS_NUM;
            if is_simple {
                Ok(WordEntryWithString::simple(
                    row_id as u32,
                    row[0].to_string(),
                ))
            } else {
                WordEntryWithString::new(row_id as u32, row, is_system)
            }
        })
        .collect::<Result<Vec<WordEntryWithString>, _>>()?;

    let mut word_entry_map: BTreeMap<String, Vec<WordEntry>> = BTreeMap::new();
    for entry in entries {
        word_entry_map
            .entry(entry.surface)
            .or_default()
            .push(entry.word_entry);
    }

    Ok(word_entry_map)
}

pub fn build_prefix_dict(
    word_entry_map: WordEntryMap,
    is_system: bool,
) -> LinderaResult<PrefixDict> {
    let mut id = 0u32;

    // building da
    let mut keyset: Vec<(&[u8], u32)> = vec![];
    for (key, word_entries) in &word_entry_map {
        let len = word_entries.len() as u32;
        let val = (id << 5) | len;
        keyset.push((key.as_bytes(), val));
        id += len;
    }

    let da_bytes = DoubleArrayBuilder::build(&keyset).ok_or_else(|| {
        LinderaErrorKind::Io.with_error(anyhow::anyhow!("DoubleArray build error for user dict."))
    })?;

    // building values
    let mut vals_data = Vec::<u8>::new();
    for word_entries in word_entry_map.values() {
        for word_entry in word_entries {
            word_entry
                .serialize(&mut vals_data)
                .map_err(|err| LinderaErrorKind::Serialize.with_error(anyhow::anyhow!(err)))?;
        }
    }

    Ok(PrefixDict {
        da: DoubleArray::new(da_bytes),
        vals_data,
        is_system,
    })
}

pub struct WordEntryWithString {
    surface: String,
    word_entry: WordEntry,
}

impl WordEntryWithString {
    pub fn new(row_id: u32, row: &[String], is_system: bool) -> LinderaResult<Self> {
        Ok(Self {
            surface: row[0].to_string(),
            word_entry: WordEntry {
                word_id: WordId(row_id, is_system),
                word_cost: i16::from_str(row[3].trim()).map_err(|_err| {
                    LinderaErrorKind::Parse.with_error(anyhow::anyhow!("failed to parse word_cost"))
                })?,
                left_id: u16::from_str(row[1].trim()).map_err(|_err| {
                    LinderaErrorKind::Parse.with_error(anyhow::anyhow!("failed to parse cost_id"))
                })?,
                right_id: u16::from_str(row[2].trim()).map_err(|_err| {
                    LinderaErrorKind::Parse.with_error(anyhow::anyhow!("failed to parse cost_id"))
                })?,
            },
        })
    }
    pub fn simple(row_id: u32, string: String) -> Self {
        Self {
            surface: string,
            word_entry: WordEntry {
                word_id: WordId(row_id, false),
                word_cost: SIMPLE_WORD_COST,
                left_id: SIMPLE_CONTEXT_ID,
                right_id: SIMPLE_CONTEXT_ID,
            },
        }
    }
}

pub fn build_words<S: DictionarySerializer + Send + Sync>(
    serializer: &S,
    rows: &Vec<Vec<String>>,
    is_system: bool,
) -> LinderaResult<(Vec<u8>, Vec<u8>)> {
    let mut words = rows
        .par_iter()
        .map(|row| {
            if is_system || row.len() >= DETAILED_USERDIC_FIELDS_NUM {
                serializer.serialize(&row[4..])
            } else if row.len() == SIMPLE_USERDIC_FIELDS_NUM {
                serializer.serialize_simple(row)
            } else {
                Err(LinderaErrorKind::Content.with_error(anyhow::anyhow!(
                    "user dictionary should be a CSV with {} or {}+ fields",
                    SIMPLE_USERDIC_FIELDS_NUM,
                    DETAILED_USERDIC_FIELDS_NUM
                )))
            }
        })
        .collect::<Result<Vec<Vec<u8>>, _>>()?;

    words.insert(0, serializer.identifier().as_bytes().to_vec());

    let words_idx: Vec<usize> = words
        .iter()
        .scan(0, |acc, e| {
            let offset = *acc;
            *acc += e.len();
            Some(offset)
        })
        .collect();

    let mut words_idx_buffer = Vec::with_capacity(words_idx.len() * 4);
    // The first element is metadata, so skip it.
    for word_idx in words_idx.iter().skip(1) {
        words_idx_buffer
            .write_u32::<LittleEndian>(*word_idx as u32)
            .map_err(|err| LinderaErrorKind::Io.with_error(err))?;
    }

    let words_buffer = words.concat();

    Ok((words_idx_buffer, words_buffer))
}
