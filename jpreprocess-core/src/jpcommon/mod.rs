mod feature;
mod label;
mod word_attr;

pub use feature::*;
pub use label::Utterance;

use crate::NJDNode;

pub fn njdnodes_to_features(njd_nodes: &[NJDNode]) -> Vec<String> {
    let utterance = Utterance::from(njd_nodes);
    utterance_to_features(&utterance)
}
