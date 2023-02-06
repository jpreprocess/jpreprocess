use serde::{Serialize, Deserialize};

use crate::{accent_rule::ChainRules, pos::PartOfSpeech};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeDetails {
    pub(crate) pos: PartOfSpeech,
    //pub(crate) ctype: String,
    //pub(crate) cform: String,
    pub(crate) is_renyou: bool,
    pub(crate) orig: String,
    pub(crate) read: Option<String>,
    pub(crate) pron: Option<String>,
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
            pos: PartOfSpeech::new([details[0], details[1], details[2], details[3]]),
            //ctype: details[4].to_string(),
            //cform: details[5].to_string(),
            is_renyou: details[5].starts_with("連用"),
            chain_rule: match chain_rule {
                "*" => None,
                _ => Some(ChainRules::new(chain_rule)),
            },
            chain_flag: match details[11] {
                "1" => Some(true),
                "0" => Some(false),
                _ => None,
            },
            orig: orig.to_string(),
            read: match read {
                "*" => None,
                _ => Some(read.to_string()),
            },
            pron: match pron {
                "*" => None,
                _ => Some(pron.to_string()),
            },
            acc: 0,
            mora_size: 0,
        };

        if acc.contains("*") || !acc.contains("/") {
            return vec![node];
        }

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
                new_node.pron = match pron {
                    "*" => None,
                    _ => Some(pron.to_string()),
                };

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
