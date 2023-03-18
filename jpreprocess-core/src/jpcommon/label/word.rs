use crate::{jpcommon::word_attr::*, pronounciation::Pronounciation, NJDNode};

pub struct Word {
    pos: Option<u8>,
    ctype: Option<u8>,
    cform: Option<u8>,
    pub moras: Pronounciation,
}

impl Word {
    pub fn to_b(&self) -> String {
        format!(
            "/B:{}-{}_{}",
            Self::format_id(self.pos),
            Self::format_id(self.ctype),
            Self::format_id(self.cform)
        )
    }
    pub fn to_c(&self) -> String {
        format!(
            "/C:{}_{}+{}",
            Self::format_id(self.pos),
            Self::format_id(self.ctype),
            Self::format_id(self.cform)
        )
    }
    pub fn to_d(&self) -> String {
        format!(
            "/D:{}+{}_{}",
            Self::format_id(self.pos),
            Self::format_id(self.ctype),
            Self::format_id(self.cform)
        )
    }

    fn format_id(id: Option<u8>) -> String {
        if let Some(id) = id {
            format!("{:0>2}", id)
        } else {
            "xx".to_string()
        }
    }

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
