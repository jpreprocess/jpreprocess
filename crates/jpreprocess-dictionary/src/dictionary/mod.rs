use std::collections::BTreeMap;

use lindera_dictionary::viterbi::WordEntry;

pub(crate) type WordEntryMap = BTreeMap<String, Vec<WordEntry>>;

pub mod to_csv;
pub mod to_dict;
pub mod word_encoding;
