use std::collections::BTreeMap;

use lindera_core::word_entry::WordEntry;

pub(crate) type WordEntryMap = BTreeMap<String, Vec<WordEntry>>;

pub mod to_csv;
pub mod to_dict;
