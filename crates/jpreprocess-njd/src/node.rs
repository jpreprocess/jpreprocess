use std::fmt::Display;
use std::{fmt::Debug, str::FromStr};

use jpreprocess_core::word_entry::WordEntry;
use jpreprocess_core::{
    cform::CForm, ctype::CType, pos::*, pronunciation::Pronunciation, word_details::WordDetails,
};

use jpreprocess_core::accent_rule::ChainRule;

#[derive(Clone, PartialEq, Debug)]
pub struct NJDNode {
    string: String, //*は空文字列として扱う
    details: WordDetails,
}

impl Display for NJDNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{:?},{:?},{:?},{},{},{},{}/{},{},{}",
            self.string,
            self.details.pos,
            self.details.ctype,
            self.details.cform,
            // Ideally, this should be `self.details.orig`, but jpreprocess njdnode does not have orig
            // and in most cases, orig is the same as string.
            self.string,
            self.details.read.as_ref().unwrap_or(&"*".to_string()),
            self.details.pron,
            self.details.acc,
            self.details.mora_size,
            self.details.chain_rule,
            match self.details.chain_flag {
                Some(true) => 1,
                Some(false) => 0,
                None => -1,
            }
        )
    }
}

impl NJDNode {
    pub fn new_single(s: &str) -> Self {
        let nodes = Self::load_csv(s);
        if nodes.len() == 1 {
            nodes.into_iter().next().unwrap()
        } else {
            panic!("input string must contain exactly one node.");
        }
    }
    pub fn load_csv(s: &str) -> Vec<Self> {
        let splited = {
            let mut splited: Vec<&str> = s.split(",").collect();
            splited.resize(13, "");
            splited
        };
        Self::load_str(splited[0], &splited[1..splited.len()])
    }
    pub fn load_str(string: &str, details: &[&str]) -> Vec<Self> {
        let entry = WordEntry::load(details).unwrap();
        Self::load(string, entry)
    }
    pub fn load(string: &str, entry: WordEntry) -> Vec<Self> {
        entry
            .get_with_string(string)
            .into_iter()
            .map(|(string, details)| Self { string, details })
            .collect()
    }

    pub fn transfer_from(&mut self, node: &mut Self) {
        self.string.push_str(&node.string);
        self.add_mora_size(node.details.mora_size);
        if let Some(add) = &node.details.read {
            if let Some(read) = &mut self.details.read {
                read.push_str(add);
            } else {
                self.details.read = Some(add.to_string());
            }
        }
        self.get_pron_mut().transfer_from(&node.details.pron);
        node.unset_pron();
    }

    pub fn get_chain_flag(&self) -> Option<bool> {
        self.details.chain_flag
    }
    pub fn set_chain_flag(&mut self, chain_flag: bool) {
        self.details.chain_flag = Some(chain_flag);
    }

    pub fn get_chain_rule(&self, pos: &POS) -> Option<&ChainRule> {
        self.details.chain_rule.get_rule(pos)
    }
    pub fn unset_chain_rule(&mut self) {
        self.details.chain_rule.unset();
    }

    pub fn get_pos(&self) -> &POS {
        &self.details.pos
    }
    pub fn get_pos_mut(&mut self) -> &mut POS {
        &mut self.details.pos
    }

    pub fn is_renyou(&self) -> bool {
        self.details.cform.is_renyou()
    }
    pub fn get_ctype(&self) -> &CType {
        &self.details.ctype
    }
    pub fn get_cform(&self) -> &CForm {
        &self.details.cform
    }

    pub fn get_string(&self) -> &str {
        self.string.as_str()
    }
    pub fn replace_string(&mut self, new_string: &str) {
        self.string = new_string.to_string();
    }
    pub fn ensure_orig(&mut self) {}

    pub fn get_read(&self) -> Option<&str> {
        self.details.read.as_ref().map(|read| read.as_str())
    }
    pub fn set_read(&mut self, read: &str) {
        self.details.read = Some(read.to_string());
    }
    pub fn unset_read(&mut self) {
        self.details.read = None;
    }

    pub fn get_acc(&self) -> i32 {
        self.details.acc
    }
    pub fn set_acc(&mut self, acc: i32) {
        self.details.acc = acc;
    }

    pub fn get_mora_size(&self) -> i32 {
        self.details.mora_size
    }
    pub fn set_mora_size(&mut self, mora_size: i32) {
        self.details.mora_size = mora_size;
    }
    pub fn add_mora_size(&mut self, mora_size: i32) {
        self.details.mora_size += mora_size;
        if self.details.mora_size < 0 {
            self.details.mora_size = 0;
        }
    }

    pub fn set_pron_by_str(&mut self, pron: &str) {
        self.details.pron = Pronunciation::from_str(pron).unwrap();
    }
    pub fn get_pron(&self) -> &Pronunciation {
        &self.details.pron
    }
    pub fn get_pron_mut(&mut self) -> &mut Pronunciation {
        &mut self.details.pron
    }
    pub fn set_pron(&mut self, pron: Pronunciation) {
        self.details.pron = pron;
    }
    pub fn unset_pron(&mut self) {
        self.details.pron = Pronunciation::default();
    }
}

#[cfg(test)]
mod tests {
    use super::NJDNode;

    #[test]
    fn single_node() {
        let node = NJDNode::new_single("．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1");
        assert_eq!(node.string, "．");
        assert_eq!(node.is_renyou(), false);

        assert_eq!(
            node.to_string(),
            "．,Meishi(Setsubi(Josuushi)),None,None,．,テン,テン,0/2,*,-1"
        )
    }

    #[test]
    fn multiple_nodes() {
        let nodes = NJDNode::load_csv("あーあ,感動詞,*,*,*,*,*,あー:あ,アー:ア,アー:ア,1/2:1/1,C1");
        assert_eq!(nodes.len(), 2);

        assert_eq!(
            nodes[0].to_string(),
            "あー,Kandoushi,None,None,あー,アー,アー,1/2,C1,-1"
        );
        assert_eq!(
            nodes[1].to_string(),
            "あ,Kandoushi,None,None,あ,ア,ア,1/1,C1,0"
        );
    }
}
