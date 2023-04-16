use once_cell::sync::Lazy;

use crate::{
    cform::CForm, ctype::CType, word_details::WordDetails, pos::*, pronounciation::Pronounciation,
};

pub static UNK: Lazy<WordDetails> = Lazy::new(|| WordDetails {
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
