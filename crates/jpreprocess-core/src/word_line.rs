use std::{borrow::Cow, fmt::Display};

/// A struct to represent a line in a word dictionary file.
///
/// > [!WARNING]
/// > This struct is experimental and may change in the future.
#[derive(Clone, PartialEq, Debug)]
pub struct WordDetailsLine<'a> {
    pub pos: Cow<'a, str>,
    pub pos_group1: Cow<'a, str>,
    pub pos_group2: Cow<'a, str>,
    pub pos_group3: Cow<'a, str>,
    pub ctype: Cow<'a, str>,
    pub cform: Cow<'a, str>,
    pub orig: Cow<'a, str>,
    pub read: Cow<'a, str>,
    pub pron: Cow<'a, str>,
    pub acc_morasize: Cow<'a, str>,
    pub chain_rule: Cow<'a, str>,
    pub chain_flag: Cow<'a, str>,
}

impl<'a> WordDetailsLine<'a> {
    pub fn from_strs(details: &[&'a str]) -> Self {
        assert_eq!(details.len(), 12, "line must have exactly 12 columns");

        Self {
            pos: details[0].into(),
            pos_group1: details[1].into(),
            pos_group2: details[2].into(),
            pos_group3: details[3].into(),
            ctype: details[4].into(),
            cform: details[5].into(),
            orig: details[6].into(),
            read: details[7].into(),
            pron: details[8].into(),
            acc_morasize: details[9].into(),
            chain_rule: details[10].into(),
            chain_flag: details[11].into(),
        }
    }

    pub fn to_str_vec(&self, orig: String) -> [String; 12] {
        [
            self.pos.to_string(),
            self.pos_group1.to_string(),
            self.pos_group2.to_string(),
            self.pos_group3.to_string(),
            self.ctype.to_string(),
            self.cform.to_string(),
            orig,
            self.read.to_string(),
            self.pron.to_string(),
            self.acc_morasize.to_string(),
            self.chain_rule.to_string(),
            self.chain_flag.to_string(),
        ]
    }
}

impl Display for WordDetailsLine<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            self.pos,
            self.pos_group1,
            self.pos_group2,
            self.pos_group3,
            self.ctype,
            self.cform,
            self.orig,
            self.read,
            self.pron,
            self.acc_morasize,
            self.chain_rule,
            self.chain_flag
        )
    }
}
