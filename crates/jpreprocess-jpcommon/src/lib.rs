mod feature;
mod label;
mod word_attr;

pub use feature::*;
use jlabel::Label;
pub use label::*;

use jpreprocess_njd::NJDNode;

/// Converts NJDNode to fullcontext label
pub fn njdnodes_to_features(njd_nodes: &[NJDNode]) -> Vec<Label> {
    let utterance = Utterance::from(njd_nodes);
    utterance_to_features(&utterance)
}
