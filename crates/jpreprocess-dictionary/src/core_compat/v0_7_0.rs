//! This file is a compatibility layer for jpreprocess-core of v0.7.0 and below and v0.8.0 and over.
//!
//! If you change POS or other inner structures, you need to change this file as well.

use jpreprocess_core::{
    accent_rule::ChainRules, cform::CForm, ctype::CType, pos::POS, pronunciation::Pronunciation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum WordEntry {
    Single(WordDetails),
    Multiple(Vec<(String, WordDetails)>),
}

impl From<WordEntry> for jpreprocess_core::word_entry::WordEntry {
    fn from(value: WordEntry) -> Self {
        match value {
            WordEntry::Single(details) => {
                jpreprocess_core::word_entry::WordEntry::Single(details.into())
            }
            WordEntry::Multiple(details) => jpreprocess_core::word_entry::WordEntry::Multiple(
                details
                    .into_iter()
                    .map(|(s, details)| (s, details.into()))
                    .collect(),
            ),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordDetails {
    pub pos: POS,
    pub ctype: CType,
    pub cform: CForm,
    pub read: Option<String>,
    pub pron: Pronunciation,
    pub acc: i32,
    pub mora_size: i32,
    pub chain_rule: ChainRules,
    pub chain_flag: Option<bool>,
}

impl From<WordDetails> for jpreprocess_core::word_details::WordDetails {
    fn from(value: WordDetails) -> Self {
        jpreprocess_core::word_details::WordDetails {
            pos: value.pos,
            ctype: value.ctype,
            cform: value.cform,
            read: value.read,
            pron: value.pron,
            acc: value.acc,
            mora_size: value.mora_size,
            chain_rule: value.chain_rule,
            chain_flag: value.chain_flag,
        }
    }
}
