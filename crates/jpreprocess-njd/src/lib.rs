mod contrib;
mod node;
mod open_jtalk;

use jpreprocess_core::JPreprocessResult;
use jpreprocess_dictionary::DictionaryFetcher;
use jpreprocess_window::{IterQuintMut, IterQuintMutTrait};
use lindera_tokenizer::token::Token;

pub use contrib::*;
pub use node::*;
pub use open_jtalk::*;

#[derive(Clone, Debug, PartialEq)]
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
        use open_jtalk::*;

        pronunciation::njd_set_pronunciation(self);
        digit_sequence::njd_digit_sequence(self);
        digit::njd_set_digit(self);
        accent_phrase::njd_set_accent_phrase(self);
        accent_type::njd_set_accent_type(self);
        unvoiced_vowel::njd_set_unvoiced_vowel(self);
        // long vowel estimator is deprecated
        // long_vowel::njd_set_long_vowel(self);
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
