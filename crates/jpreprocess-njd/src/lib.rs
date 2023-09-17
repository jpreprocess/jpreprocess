mod njd_set;
mod node;

use jpreprocess_core::JPreprocessResult;
use jpreprocess_dictionary::DictionaryFetcher;
use jpreprocess_window::{IterQuintMut, IterQuintMutTrait};
use lindera_tokenizer::token::Token;

pub use njd_set::*;
pub use node::*;

#[derive(Debug)]
pub struct NJD {
    pub nodes: Vec<NJDNode>,
}

impl NJD {
    pub fn remove_silent_node(&mut self) {
        self.nodes.retain(|node| !node.get_pron().is_empty())
    }

    pub fn from_tokens<S: DictionaryFetcher>(
        tokens: &[Token],
        fetcher: &S,
    ) -> JPreprocessResult<Self> {
        let nodes = fetcher
            .get_word_vectored(tokens)?
            .into_iter()
            .zip(tokens)
            .flat_map(|(word_entry, token)| NJDNode::load(token.text, word_entry))
            .collect();
        Ok(Self { nodes })
    }

    pub fn from_strings(njd_features: Vec<String>) -> Self {
        Self {
            nodes: njd_features
                .iter()
                .flat_map(|feature| NJDNode::load_csv(feature))
                .collect(),
        }
    }

    pub fn preprocess(&mut self) {
        njd_set::preprocess_njd(self)
    }
}

impl IterQuintMutTrait for NJD {
    type Item = NJDNode;
    fn iter_quint_mut(&mut self) -> IterQuintMut<'_, Self::Item> {
        IterQuintMut::new(&mut self.nodes)
    }
    fn iter_quint_mut_range(&mut self, start: usize, end: usize) -> IterQuintMut<'_, Self::Item> {
        IterQuintMut::new(&mut self.nodes[start..end])
    }
}

impl From<NJD> for Vec<String> {
    fn from(njd: NJD) -> Self {
        njd.nodes.into_iter().map(|node| node.to_string()).collect()
    }
}
