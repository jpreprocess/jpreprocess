use once_cell::sync::Lazy;

use crate::{
    cform::CForm, ctype::CType, node_details::NodeDetails, pos::*, pronounciation::Pronounciation,
};

pub static UNK: Lazy<NodeDetails> = Lazy::new(|| NodeDetails {
    pos: POS::Meishi(Meishi::None),
    ctype: CType::None,
    cform: CForm::None,
    orig: "*".to_string(),
    read: None,
    pron: Pronounciation::default(),
    acc: 0,
    mora_size: 0,
    chain_rule: None,
    chain_flag: None,
});
