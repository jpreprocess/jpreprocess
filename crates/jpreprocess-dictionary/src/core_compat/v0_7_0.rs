//! This file is a compatibility layer for jpreprocess-core of v0.7.0 and below and v0.8.0 and over.
//!
//! If you change POS or other inner structures, you need to change this file as well.

use jpreprocess_core::{
    accent_rule::AccentType, cform::CForm, ctype::CType, pos::POS, pronunciation::Mora,
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
            pron: jpreprocess_core::pronunciation::Pronunciation::new(
                value.pron.0,
                value.acc as usize,
            ),
            chain_rule: value.chain_rule.into(),
            chain_flag: value.chain_flag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pronunciation(Vec<Mora>);

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainRules {
    default: Option<ChainRule>,
    doushi: Option<ChainRule>,
    joshi: Option<ChainRule>,
    keiyoushi: Option<ChainRule>,
    meishi: Option<ChainRule>,
}

impl From<ChainRules> for jpreprocess_core::accent_rule::ChainRules {
    fn from(value: ChainRules) -> Self {
        jpreprocess_core::accent_rule::ChainRules {
            default: value.default.map(|rule| rule.into()),
            doushi: value.doushi.map(|rule| rule.into()),
            joshi: value.joshi.map(|rule| rule.into()),
            keiyoushi: value.keiyoushi.map(|rule| rule.into()),
            meishi: value.meishi.map(|rule| rule.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainRule {
    pub accent_type: AccentType,
    pub add_type: i32,
}

impl From<ChainRule> for jpreprocess_core::accent_rule::ChainRule {
    fn from(value: ChainRule) -> Self {
        jpreprocess_core::accent_rule::ChainRule {
            accent_type: value.accent_type,
            add_type: value.add_type as isize,
        }
    }
}
