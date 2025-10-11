use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    accent_rule::ChainRules, cform::CForm, ctype::CType, pos::POS, pronunciation::Pronunciation,
    word_line::WordDetailsLine, JPreprocessResult,
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct WordDetails {
    pub pos: POS,
    pub ctype: CType,
    pub cform: CForm,
    pub read: Option<String>,
    pub pron: Pronunciation,
    pub chain_rule: ChainRules,
    pub chain_flag: Option<bool>,
}

impl WordDetails {
    pub fn load(details: &[&str]) -> JPreprocessResult<Self> {
        WordDetailsLine::from_strs(details).try_into()
    }

    pub fn extend_splited(
        &mut self,
        read: &str,
        pron: &str,
        acc_morasize: &str,
    ) -> JPreprocessResult<()> {
        self.read = match read {
            "*" => None,
            _ => Some(read.to_string()),
        };
        self.pron = Pronunciation::parse_csv_pron(pron, acc_morasize)?;
        self.chain_flag = Some(false);
        Ok(())
    }

    pub fn to_str_vec(&self, orig: String) -> [String; 9] {
        let line = WordDetailsLine::from(self);

        [
            format!(
                "{},{},{},{}",
                line.pos, line.pos_group1, line.pos_group2, line.pos_group3
            ),
            line.ctype.to_string(),
            line.cform.to_string(),
            // Ideally, this should be `self.orig`, but jpreprocess njdnode does not have orig
            // and in most cases, orig is the same as string.
            orig,
            line.read.to_string(),
            line.pron.to_string(),
            line.acc_morasize.to_string(),
            line.chain_rule.to_string(),
            line.chain_flag.to_string(),
        ]
    }
}

impl TryFrom<WordDetailsLine> for WordDetails {
    type Error = crate::JPreprocessError;
    fn try_from(value: WordDetailsLine) -> Result<WordDetails, Self::Error> {
        // orig: not used

        Ok(Self {
            pos: POS::from_strs(
                &value.pos,
                &value.pos_group1,
                &value.pos_group2,
                &value.pos_group3,
            )?,
            ctype: CType::from_str(&value.ctype)?,
            cform: CForm::from_str(&value.cform)?,
            chain_rule: ChainRules::new(&value.chain_rule),
            chain_flag: match value.chain_flag.as_ref() {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            },
            read: match value.read.as_ref() {
                "*" => None,
                _ => Some(value.read.to_string()),
            },
            pron: Pronunciation::parse_csv_pron(&value.pron, &value.acc_morasize)?,
        })
    }
}

impl From<&WordDetails> for WordDetailsLine {
    fn from(value: &WordDetails) -> Self {
        let pos = value.pos.to_string();
        let pos_parts: Vec<&str> = pos.split(',').collect();
        assert_eq!(pos_parts.len(), 4, "POS must have exactly 4 parts");

        Self {
            pos: pos_parts[0].to_string().into(),
            pos_group1: pos_parts[1].to_string().into(),
            pos_group2: pos_parts[2].to_string().into(),
            pos_group3: pos_parts[3].to_string().into(),
            ctype: value.ctype.to_string().into(),
            cform: value.cform.to_string().into(),
            orig: "*".into(), // orig is not stored in WordDetails
            read: value.read.as_deref().unwrap_or("*").to_string().into(),
            pron: value.pron.to_string().into(),
            acc_morasize: format!("{}/{}", value.pron.accent(), value.pron.mora_size()).into(),
            chain_rule: value.chain_rule.to_string().into(),
            chain_flag: match value.chain_flag {
                Some(true) => "1",
                Some(false) => "0",
                None => "-1",
            }
            .into(),
        }
    }
}
