use std::fmt::Debug;
use std::fmt::Display;

use jpreprocess_core::word_entry::WordEntry;
use jpreprocess_core::{
    cform::CForm, ctype::CType, pos::*, pronunciation::Pronunciation, word_details::WordDetails,
};

use jpreprocess_core::accent_rule::ChainRules;

#[derive(Clone, PartialEq, Debug)]
pub struct NJDNode {
    string: String, //*は空文字列として扱う
    details: WordDetails,
}

impl Display for NJDNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{}",
            self.string,
            self.details.to_str_vec(self.string.to_owned()).join(",")
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
            let mut splited: Vec<&str> = s.split(',').collect();
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
        if let Some(add) = &node.details.read {
            if let Some(read) = &mut self.details.read {
                read.push_str(add);
            } else {
                self.details.read = Some(add.to_string());
            }
        }
        self.get_pron_mut().transfer_from(&node.details.pron);
        node.reset();
    }
    pub fn reset(&mut self) {
        self.string.clear();
        self.details = WordDetails::default();
    }
}

/// Getters and setters
impl NJDNode {
    pub fn get_chain_flag(&self) -> Option<bool> {
        self.details.chain_flag
    }
    pub fn set_chain_flag(&mut self, chain_flag: bool) {
        self.details.chain_flag = Some(chain_flag);
    }

    pub fn get_chain_rule(&self) -> &ChainRules {
        &self.details.chain_rule
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

    pub fn get_read(&self) -> Option<&str> {
        self.details.read.as_deref()
    }
    pub fn set_read(&mut self, read: &str) {
        self.details.read = Some(read.to_string());
    }
    pub fn unset_read(&mut self) {
        self.details.read = None;
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
}

#[cfg(test)]
mod tests {
    use super::NJDNode;

    #[test]
    fn single_node() {
        let node = NJDNode::new_single("．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1");
        assert_eq!(node.string, "．");
        assert!(!node.is_renyou());

        assert_eq!(
            node.to_string(),
            "．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1"
        )
    }

    #[test]
    fn multiple_nodes() {
        let nodes = NJDNode::load_csv("あーあ,感動詞,*,*,*,*,*,あー:あ,アー:ア,アー:ア,1/2:1/1,C1");
        assert_eq!(nodes.len(), 2);

        assert_eq!(
            nodes[0].to_string(),
            "あー,感動詞,*,*,*,*,*,あー,アー,アー,1/2,C1,-1"
        );
        assert_eq!(
            nodes[1].to_string(),
            "あ,感動詞,*,*,*,*,*,あ,ア,ア,1/1,C1,0"
        );
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<NJDNode>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<NJDNode>();
    }
}
