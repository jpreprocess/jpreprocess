use std::str::FromStr;

use jpreprocess_core::{
    accent_rule::ChainRules,
    cform::CForm,
    ctype::CType,
    pos::POS,
    pronunciation::{Pronunciation, PronunciationParseError},
    word_details::WordDetails,
    word_entry::WordEntry,
    JPreprocessError,
};
use jpreprocess_njd::NJDNode;
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};
use serde::{Deserialize, Serialize};

use crate::into_runtime_error;

#[derive(FromPyObject)]
pub enum StringOrArray {
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "list[str]")]
    Array(Vec<String>),
}
impl IntoPy<PyObject> for StringOrArray {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.to_object(py)
    }
}
impl ToPyObject for StringOrArray {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Self::String(s) => s.to_object(py),
            Self::Array(arr) => arr.to_object(py),
        }
    }
}
impl StringOrArray {
    pub(crate) fn join(&mut self, sep: &'static str) {
        if let Self::Array(array) = self {
            *self = Self::String(array.join(sep));
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NjdObject {
    string: String,
    pos: String,
    pos_group1: String,
    pos_group2: String,
    pos_group3: String,
    ctype: String,
    cform: String,
    orig: String,
    read: String,
    pron: String,
    acc: usize,
    mora_size: usize,
    chain_rule: String,
    chain_flag: i32,
}

impl From<NJDNode> for NjdObject {
    fn from(value: NJDNode) -> Self {
        let pos = value.get_pos().to_string();
        let pos_strings: Vec<&str> = pos.split(',').collect();
        Self {
            string: value.get_string().to_string(),
            pos: pos_strings[0].to_string(),
            pos_group1: pos_strings[1].to_string(),
            pos_group2: pos_strings[2].to_string(),
            pos_group3: pos_strings[3].to_string(),
            ctype: value.get_ctype().to_string(),
            cform: value.get_cform().to_string(),
            orig: value.get_string().to_string(),
            read: value.get_read().unwrap_or("*").to_string(),
            pron: value.get_pron().to_string(),
            acc: value.get_pron().accent(),
            mora_size: value.get_pron().mora_size(),
            chain_rule: value.get_chain_rule().to_string(),
            chain_flag: match value.get_chain_flag() {
                Some(true) => 1,
                Some(false) => 0,
                None => -1,
            },
        }
    }
}

impl TryFrom<NjdObject> for NJDNode {
    type Error = JPreprocessError;
    fn try_from(value: NjdObject) -> Result<Self, Self::Error> {
        let details = WordDetails {
            pos: POS::from_strs(
                &value.pos,
                &value.pos_group1,
                &value.pos_group2,
                &value.pos_group3,
            )?,
            ctype: CType::from_str(&value.ctype)?,
            cform: CForm::from_str(&value.cform)?,
            read: match value.read.as_str() {
                "*" => None,
                read => Some(read.to_string()),
            },
            pron: Pronunciation::parse(&value.pron, value.acc)?,
            chain_rule: ChainRules::new(&value.chain_rule),
            chain_flag: match value.chain_flag {
                1 => Some(true),
                0 => Some(false),
                _ => None,
            },
        };
        if details.pron.mora_size() != value.mora_size {
            return Err(PronunciationParseError::MoraSizeMismatch(
                value.mora_size,
                details.pron.mora_size(),
            )
            .into());
        }
        let node = NJDNode::load(&value.string, WordEntry::Single(details));
        Ok(node[0].to_owned())
    }
}

impl IntoPy<PyObject> for NjdObject {
    fn into_py(self, py: Python<'_>) -> PyObject {
        pythonize(py, &self).expect("Failed to pythonize")
    }
}

impl<'a> FromPyObject<'a> for NjdObject {
    fn extract(ob: &'a pyo3::prelude::PyAny) -> pyo3::prelude::PyResult<Self> {
        depythonize(ob).map_err(into_runtime_error)
    }
}
