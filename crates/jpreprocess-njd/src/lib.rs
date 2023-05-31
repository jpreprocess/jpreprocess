mod njd_set;
mod node;

use jpreprocess_core::{error::JPreprocessErrorKind, word_entry::WordEntry, JPreprocessResult};
use jpreprocess_dictionary::{DictionaryTrait, JPreprocessDictionary};
use jpreprocess_window::{IterQuintMut, IterQuintMutTrait};
use lindera_tokenizer::token::Token;

pub use njd_set::proprocess_njd;
pub use node::*;

#[derive(Debug)]
pub struct NJD {
    pub nodes: Vec<NJDNode>,
}

impl NJD {
    pub fn remove_silent_node(&mut self) {
        self.nodes.retain(|node| !node.get_pron().is_empty())
    }
    pub fn from_tokens_string(tokens: Vec<Token>) -> JPreprocessResult<Self> {
        let mut nodes = Vec::new();
        for mut token in tokens {
            let text = token.text.to_string();
            let mut details_str = token.get_details().unwrap();
            let details = if details_str.len() == 1 && details_str[0] == "UNK" {
                WordEntry::default()
            } else {
                details_str.resize(13, "");
                WordEntry::load(&details_str)?
            };
            nodes.extend(NJDNode::load(&text, details));
        }
        Ok(Self { nodes })
    }

    pub fn from_tokens_dict(
        tokens: Vec<Token>,
        dict: &JPreprocessDictionary,
    ) -> JPreprocessResult<Self> {
        let mut nodes = Vec::new();
        for token in tokens {
            let text = token.text.to_string();
            let details = if !token.word_id.is_unknown() {
                let id =
                    token.word_id.0.try_into().map_err(|e| {
                        JPreprocessErrorKind::DictionaryIndexOutOfRange.with_error(e)
                    })?;
                dict.get(id)
            } else {
                None
            }
            .unwrap_or_else(|| WordEntry::default());

            nodes.extend(NJDNode::load(&text, details));
        }
        Ok(Self { nodes })
    }

    pub fn from_strings(njd_features: &[&str]) -> Self {
        Self {
            nodes: njd_features
                .iter()
                .flat_map(|feature| NJDNode::load_csv(feature))
                .collect(),
        }
    }

    pub fn proprocess(&mut self) {
        njd_set::proprocess_njd(self)
    }
}

impl IterQuintMutTrait for NJD {
    type Item = NJDNode;
    fn iter_quint_mut<'a>(&'a mut self) -> IterQuintMut<'a, Self::Item> {
        IterQuintMut::new(&mut self.nodes)
    }
    fn iter_quint_mut_range<'a>(
        &'a mut self,
        start: usize,
        end: usize,
    ) -> IterQuintMut<'a, Self::Item> {
        IterQuintMut::new(&mut self.nodes[start..end])
    }
}

impl From<NJD> for Vec<String> {
    fn from(njd: NJD) -> Self {
        njd.nodes.into_iter().map(|node| node.to_string()).collect()
    }
}
