use std::path::PathBuf;

use jpreprocess::{JPreprocess, JPreprocessConfig, SystemDictionaryConfig};
use jpreprocess_core::pos::POS;
use jpreprocess_dictionary::tokenizer::default::DefaultTokenizer;
use jpreprocess_jpcommon::njdnodes_to_features;
use jpreprocess_njd::NJDNode;

use pyo3::prelude::*;

use crate::{into_runtime_error, structs::*};

#[pyclass(name = "JPreprocess")]
pub struct JPreprocessPyBinding {
    inner: JPreprocess<DefaultTokenizer>,
}

#[pymethods]
impl JPreprocessPyBinding {
    #[new]
    #[pyo3(signature = (dictionary, user_dictionary=None))]
    fn new(dictionary: PathBuf, user_dictionary: Option<PathBuf>) -> PyResult<Self> {
        Ok(Self {
            inner: JPreprocess::from_config(JPreprocessConfig {
                dictionary: SystemDictionaryConfig::File(dictionary),
                user_dictionary: user_dictionary.map(|path| {
                    serde_json::json!({
                        "path": path,
                        "kind": "ipadic",
                    })
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
    #[pyo3(signature = (text, kana=None, join=None))]
    fn g2p(&self, text: &str, kana: Option<bool>, join: Option<bool>) -> PyResult<StringOrArray> {
        let kana = kana.unwrap_or(false);
        let join = join.unwrap_or(true);

        let prons = if kana {
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
        } else {
            let labels = self
                .inner
                .extract_fullcontext(text)
                .map_err(into_runtime_error)?;
            let label_len = labels.len();
            labels
                .into_iter()
                .skip(1)
                .take(label_len - 2)
                .map(|label| label.phoneme.c.unwrap())
                .collect()
        };
        let mut result = StringOrArray::Array(prons);
        if join {
            result.join(if kana { "" } else { " " });
        }
        Ok(result)
    }
}
