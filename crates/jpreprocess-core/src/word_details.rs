use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    accent_rule::ChainRules, cform::CForm, ctype::CType, pos::POS, pronunciation::Pronunciation,
    JPreprocessResult,
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
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

impl WordDetails {
    pub fn load(details: &[&str]) -> JPreprocessResult<Self> {
        // let orig = details[6];
        let read = details[7];
        let pron = details[8];
        let acc_morasize = details[9];
        let chain_rule = details[10];

        let (acc, mora_size) = Self::parse_acc_morasize(acc_morasize);

        Ok(Self {
            pos: POS::from_strs(details[0], details[1], details[2], details[3])?,
            ctype: CType::from_str(details[4])?,
            cform: CForm::from_str(details[5])?,
            chain_rule: ChainRules::new(chain_rule),
            chain_flag: match details[11] {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            },
            read: match read {
                "*" => None,
                _ => Some(read.to_string()),
            },
            pron: Pronunciation::from_str(pron)?,
            acc,
            mora_size,
        })
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
        self.pron = Pronunciation::from_str(pron)?;
        let (acc, mora_size) = Self::parse_acc_morasize(acc_morasize);
        self.acc = acc;
        self.mora_size = mora_size;
        self.chain_flag = Some(false);
        Ok(())
    }

    fn parse_acc_morasize(acc_morasize: &str) -> (i32, i32) {
        match acc_morasize.split_once('/') {
            Some((acc_s, mora_size_s)) => {
                let acc = acc_s.parse().unwrap_or(0);
                let mora_size = mora_size_s.parse().unwrap_or(0);
                (acc, mora_size)
            }
            None => (0, 0),
        }
    }

    pub fn to_str_vec(&self, orig: String) -> [String; 9] {
        [
            self.pos.to_string(),
            self.ctype.to_string(),
            self.cform.to_string(),
            // Ideally, this should be `self.orig`, but jpreprocess njdnode does not have orig
            // and in most cases, orig is the same as string.
            orig,
            self.read.to_owned().unwrap_or("*".to_string()),
            self.pron.to_string(),
            format!("{}/{}", self.acc, self.mora_size),
            self.chain_rule.to_string(),
            match self.chain_flag {
                Some(true) => 1,
                Some(false) => 0,
                None => -1,
            }
            .to_string(),
        ]
    }
}
