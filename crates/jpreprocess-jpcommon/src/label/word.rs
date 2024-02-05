use jpreprocess_core::pronunciation::Pronunciation;

use jpreprocess_njd::NJDNode;

use crate::word_attr::*;

#[derive(Clone, Debug)]
pub struct Word {
    pos: Option<u8>,
    ctype: Option<u8>,
    cform: Option<u8>,
    pub moras: Pronunciation,
}

impl Word {
    pub fn count_mora(&self) -> usize {
        self.moras.mora_size()
    }
}

impl From<&NJDNode> for Word {
    fn from(njdnode: &NJDNode) -> Self {
        Self {
            pos: pos_to_id(njdnode.get_pos()),
            ctype: ctype_to_id(njdnode.get_ctype()),
            cform: cform_to_id(njdnode.get_cform()),
            moras: njdnode.get_pron().clone(),
        }
    }
}

impl From<&Word> for jlabel::Word {
    fn from(val: &Word) -> Self {
        jlabel::Word {
            pos: val.pos,
            ctype: val.ctype,
            cform: val.cform,
        }
    }
}
