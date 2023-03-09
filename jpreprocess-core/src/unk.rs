use once_cell::sync::Lazy;

use crate::{node_details::NodeDetails, pos::*, pronounciation::Pronounciation};

pub const UNK: Lazy<NodeDetails> = Lazy::new(|| NodeDetails {
    pos: POS::Meishi(Meishi::None),
    is_renyou: false,
    orig: "*".to_string(),
    read: None,
    pron: Pronounciation::default(),
    acc: 0,
    mora_size: 0,
    chain_rule: None,
    chain_flag: None,
});
