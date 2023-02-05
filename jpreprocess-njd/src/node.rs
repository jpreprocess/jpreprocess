use std::fmt::Debug;

use crate::pos::*;

use super::accent_rule::ChainRules;

#[derive(Clone, PartialEq)]
pub struct NJDNode {
    string: String, //*は空文字列として扱う
    pos: PartOfSpeech,
    ctype: String,
    cform: String,
    orig: String,
    read: Option<String>,
    pron: Option<String>,
    acc: i32,
    mora_size: i32,
    chain_rule: Option<ChainRules>,
    chain_flag: Option<bool>,
}

impl Debug for NJDNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{:?},{},{},{},{},{},{}/{},{},{}",
            self.string,
            self.pos,
            self.ctype,
            self.cform,
            self.orig,
            self.read.as_ref().unwrap_or(&"*".to_string()),
            self.pron.as_ref().unwrap_or(&"*".to_string()),
            self.acc,
            self.mora_size,
            self.chain_rule
                .as_ref()
                .map(|r| format!("{:?}", r))
                .unwrap_or("*".to_string()),
            match self.chain_flag {
                Some(true) => 1,
                Some(false) => 0,
                None => -1,
            }
        )
    }
}

impl NJDNode {
    pub fn new_single(s: &str) -> Self {
        let nodes = Self::load_str(s);
        if nodes.len() == 1 {
            nodes.into_iter().next().unwrap()
        } else {
            panic!("input string must contain exactly one node.");
        }
    }
    pub fn load_str(s: &str) -> Vec<Self> {
        let splited = {
            let mut splited: Vec<&str> = s.split(",").collect();
            splited.resize(13, "");
            splited
        };
        Self::load(splited[0], &splited[1..splited.len()])
    }
    pub fn load(string: &str, details: &[&str]) -> Vec<Self> {
        let orig = details[6];
        let read = details[7];
        let pron = details[8];
        let acc = details[9];
        let chain_rule = details[10];

        let node = Self {
            string: string.to_string(),
            pos: PartOfSpeech::new([details[0], details[1], details[2], details[3]]),
            ctype: details[4].to_string(),
            cform: details[5].to_string(),
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

        let orig_splited: Vec<(&str, usize, usize)> = orig
            .split(":")
            .scan(0, |len, orig| {
                *len += orig.len();
                Some((*len, orig))
            })
            .enumerate()
            .map(|(i, (len, orig))| (orig, i, len))
            .collect();
        let splited_len = orig_splited.len();

        orig_splited
            .into_iter()
            .zip(acc.split(":"))
            .zip(read.split(":"))
            .zip(pron.split(":"))
            .map(|((((orig, i, len), acc_morasize), read), pron)| {
                let mut new_node = node.clone();

                if i > 0 {
                    new_node.chain_flag = Some(false);
                }
                new_node.orig = orig.to_string();
                new_node.string = if i + 1 < splited_len {
                    orig.to_string()
                } else {
                    string[len - orig.len()..string.len()].to_string()
                };
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

    pub fn transfer_from(&mut self, node: &mut Self) {
        self.string.push_str(&node.string);
        self.orig.push_str(&node.orig);
        if let Some(read) = &node.read {
            self.add_read(read);
        }
        if let Some(pron) = &node.pron {
            self.add_pron(pron);
        }
        self.add_mora_size(node.mora_size);
        node.unset_pron();
    }

    pub fn get_chain_flag(&self) -> Option<bool> {
        self.chain_flag
    }
    pub fn set_chain_flag(&mut self, chain_flag: bool) {
        self.chain_flag = Some(chain_flag);
    }

    pub fn get_chain_rule(&self) -> Option<&ChainRules> {
        self.chain_rule.as_ref()
    }
    pub fn unset_chain_rule(&mut self) {
        self.chain_rule = None;
    }

    pub fn get_pos(&self) -> &PartOfSpeech {
        &self.pos
    }
    pub fn get_pos_mut(&mut self) -> &mut PartOfSpeech {
        &mut self.pos
    }

    pub fn is_renyou(&self) -> bool {
        self.cform.starts_with("連用")
    }

    pub fn get_string(&self) -> &str {
        self.string.as_str()
    }
    pub fn replace_string(&mut self, new_string: &str) {
        self.orig = new_string.to_string();
        self.string = new_string.to_string();
    }
    pub fn ensure_orig(&mut self) {
        if self.orig == "*" {
            self.orig = self.string.clone();
        }
    }

    pub fn get_read(&self) -> Option<&str> {
        self.read.as_ref().map(|read| read.as_str())
    }
    pub fn set_read(&mut self, read: &str) {
        self.read = Some(read.to_string());
    }
    pub fn unset_read(&mut self) {
        self.read = None;
    }
    pub fn add_read(&mut self, add: &str) {
        if let Some(read) = &mut self.read {
            read.push_str(add);
        } else {
            self.read = Some(add.to_string());
        }
    }

    pub fn get_acc(&self) -> i32 {
        self.acc
    }
    pub fn set_acc(&mut self, acc: i32) {
        self.acc = acc;
    }

    pub fn get_mora_size(&self) -> i32 {
        self.mora_size
    }
    pub fn set_mora_size(&mut self, mora_size: i32) {
        self.mora_size = mora_size;
    }
    pub fn add_mora_size(&mut self, mora_size: i32) {
        self.mora_size += mora_size;
        if self.mora_size < 0 {
            self.mora_size = 0;
        }
    }

    pub fn get_pron(&self) -> Option<&str> {
        self.pron.as_ref().map(|pron| pron.as_str())
    }
    pub fn set_pron(&mut self, pron: &str) {
        self.pron = Some(pron.to_string());
    }
    pub fn unset_pron(&mut self) {
        self.pron = None;
    }
    pub fn add_pron(&mut self, add: &str) {
        if let Some(pron) = &mut self.pron {
            pron.push_str(add);
        } else {
            self.pron = Some(add.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pos::*;

    use super::NJDNode;

    #[test]
    fn load_single_node() {
        let node = NJDNode::new_single("．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1");
        assert_eq!(node.string, "．");
        assert_eq!(node.get_pos().get_group0(), Group0::Meishi);
        assert_eq!(node.get_pos().get_group1(), Group1::Setsubi);
        assert_eq!(node.get_pos().get_group3(), Group3::Others);
        assert_eq!(node.ctype, "*");
        assert_eq!(node.cform, "*");
        assert_eq!(node.orig, "．");
        assert_eq!(node.read.unwrap(), "テン");
        assert_eq!(node.pron.unwrap(), "テン");
        assert_eq!(node.acc, 0);
        assert_eq!(node.mora_size, 2);
        assert_eq!(node.chain_rule.is_none(), true);
        assert_eq!(node.chain_flag, None);
    }

    #[test]
    fn load_multiple_nodes() {
        let nodes = NJDNode::load_str("あーあ,感動詞,*,*,*,*,*,あー:あ,アー:ア,アー:ア,1/2:1/1,C1");
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].string, "あー");
        assert_eq!(nodes[1].string, "あ");
        assert_eq!(nodes[0].orig, "あー");
        assert_eq!(nodes[1].orig, "あ");
        assert_eq!(nodes[0].acc, 1);
        assert_eq!(nodes[1].acc, 1);
        assert_eq!(nodes[0].mora_size, 2);
        assert_eq!(nodes[1].mora_size, 1);
    }
}
