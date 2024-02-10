mod njd;

use std::path::PathBuf;

use ::jpreprocess::{
    DefaultFetcher, DictionaryKind, JPreprocess, JPreprocessConfig, SystemDictionaryConfig,
    UserDictionaryConfig,
};
use jpreprocess_core::pos::POS;
use jpreprocess_jpcommon::njdnodes_to_features;
use jpreprocess_njd::NJDNode;
use njd::NjdObject;
use pyo3::{exceptions::PyRuntimeError, prelude::*};

#[pymodule]
fn jpreprocess(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<JPreprocessPyBinding>()?;
    m.add("JPREPROCESS_VERSION", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[derive(FromPyObject)]
enum StringOrArray {
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
    pub(crate) fn join(&mut self) {
        if let Self::Array(array) = self {
            *self = Self::String(array.join(""));
        }
    }
}

pub fn into_runtime_error<E: ToString>(err: E) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

#[pyclass(name = "JPreprocess")]
struct JPreprocessPyBinding {
    inner: JPreprocess<DefaultFetcher>,
}

#[pymethods]
impl JPreprocessPyBinding {
    #[new]
    fn new(dictionary: PathBuf, user_dictionary: Option<PathBuf>) -> PyResult<Self> {
        Ok(Self {
            inner: JPreprocess::from_config(JPreprocessConfig {
                dictionary: SystemDictionaryConfig::File(dictionary),
                user_dictionary: user_dictionary.map(|u| UserDictionaryConfig {
                    path: u,
                    kind: Some(DictionaryKind::IPADIC),
                }),
            })
            .map_err(into_runtime_error)?,
        })
    }
    fn run_frontend(&self, text: &str) -> PyResult<Vec<NjdObject>> {
        let mut njd = self.inner.text_to_njd(text).map_err(into_runtime_error)?;
        njd.preprocess();
        Ok(njd.nodes.into_iter().map(|n| n.into()).collect())
    }
    fn make_label(&self, njd_features: Vec<NjdObject>) -> PyResult<Vec<String>> {
        let nodes = njd_features
            .into_iter()
            .map(|node| node.try_into())
            .collect::<Result<Vec<NJDNode>, _>>()
            .map_err(into_runtime_error)?;
        Ok(njdnodes_to_features(&nodes)
            .into_iter()
            .map(|l| l.to_string())
            .collect())
    }
    fn extract_fullcontext(&self, text: &str) -> PyResult<Vec<String>> {
        let labels = self
            .inner
            .extract_fullcontext(text)
            .map_err(into_runtime_error)?;
        Ok(labels.into_iter().map(|l| l.to_string()).collect())
    }
    fn g2p(&self, text: &str, kana: Option<bool>, join: Option<bool>) -> PyResult<StringOrArray> {
        let kana = kana.unwrap_or(false);
        let join = join.unwrap_or(true);

        let prons = if kana {
            let labels = self
                .inner
                .extract_fullcontext(text)
                .map_err(into_runtime_error)?;
            labels
                .into_iter()
                .map(|label| label.phoneme.c.unwrap())
                .collect()
        } else {
            let mut njd = self.inner.text_to_njd(text).map_err(into_runtime_error)?;
            njd.preprocess();
            njd.nodes
                .iter()
                .map(|node| {
                    let mut p = if matches!(node.get_pos(), POS::Kigou(_)) {
                        node.get_string().to_string()
                    } else {
                        node.get_pron().to_string()
                    };
                    p = p.replace('’', "");
                    p
                })
                .collect()
        };
        let mut result = StringOrArray::Array(prons);
        if join {
            result.join();
        }
        Ok(result)
    }
}
