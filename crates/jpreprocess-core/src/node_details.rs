use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    accent_rule::ChainRules, cform::CForm, ctype::CType, pos::POS, pronounciation::Pronounciation,
};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDetails {
    pub(crate) pos: POS,
    pub(crate) ctype: CType,
    pub(crate) cform: CForm,
    pub(crate) orig: String,
    pub(crate) read: Option<String>,
    pub(crate) pron: Pronounciation,
    pub(crate) acc: i32,
    pub(crate) mora_size: i32,
    pub(crate) chain_rule: Option<ChainRules>,
    pub(crate) chain_flag: Option<bool>,
}

impl NodeDetails {
    pub fn load(details: &[&str]) -> Vec<Self> {
        let orig = details[6];
        let read = details[7];
        let pron = details[8];
        let acc = details[9];
        let chain_rule = details[10];

        let node = Self {
            pos: POS::from_strs(details[0], details[1], details[2], details[3]).unwrap(),
            ctype: CType::from_str(details[4]).unwrap(),
            cform: CForm::from_str(details[5]).unwrap(),
            chain_rule: match chain_rule {
                "*" => None,
                _ => Some(ChainRules::new(chain_rule)),
            },
            chain_flag: match details[11] {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            },
            orig: "".to_string(),
            read: None,
            pron: Pronounciation::default(),
            acc: 0,
            mora_size: 0,
        };

        let orig_splited: Vec<(&str, usize)> = orig
            .split(":")
            .enumerate()
            .map(|(i, orig)| (orig, i))
            .collect();

        orig_splited
            .into_iter()
            .zip(acc.split(":"))
            .zip(read.split(":"))
            .zip(pron.split(":"))
            .map(|((((orig, i), acc_morasize), read), pron)| {
                let mut new_node = node.clone();

                if i > 0 {
                    new_node.chain_flag = Some(false);
                }
                new_node.orig = orig.to_string();
                new_node.read = match read {
                    "*" => None,
                    _ => Some(read.to_string()),
                };
                new_node.pron = Pronounciation::from_str(pron).unwrap();

                match acc_morasize.split_once("/") {
                    Some((acc_s, mora_size_s)) => {
                        if let Ok(acc) = acc_s.parse() {
                            new_node.acc = acc;
                        }
                        if let Ok(mora_size) = mora_size_s.parse() {
                            new_node.mora_size = mora_size;
                        }
                    }
                    None => (),
                }

                new_node
            })
            .collect()
    }
}
