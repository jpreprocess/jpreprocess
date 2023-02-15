mod dictionary;

pub use dictionary::*;
use jpreprocess_njd::node_details::NodeDetails;

use std::path::PathBuf;

use jpreprocess_core::JPreprocessResult;

pub struct LinderaDict(Dictionary);

impl LinderaDict {
    pub fn load(dir: PathBuf) -> JPreprocessResult<Dictionary> {
        Dictionary::load(dir.join("dict.words"), dir.join("dict.wordsidx"))
    }
    pub fn get(&self, index: usize) -> Option<Vec<String>> {
        self.0
            .get(index)
            .and_then(|data| bincode::deserialize(data).ok())
    }
}

pub struct JPreprocessDict(Dictionary);

impl JPreprocessDict {
    pub fn load(dir: PathBuf) -> JPreprocessResult<Dictionary> {
        Dictionary::load(
            dir.join("jpreprocess.words"),
            dir.join("jpreprocess.wordsidx"),
        )
    }
    pub fn get(&self, index: usize) -> Option<Vec<NodeDetails>> {
        self.0
            .get(index)
            .and_then(|data| bincode::deserialize(data).ok())
    }
}
