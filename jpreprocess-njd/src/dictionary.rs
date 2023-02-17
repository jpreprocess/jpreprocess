use std::path::PathBuf;

use jpreprocess_core::JPreprocessResult;
use jpreprocess_dictionary::*;

use crate::node_details::NodeDetails;


pub struct JPreprocessDict(Dictionary);
impl DictionaryTrait for JPreprocessDict {
    type StoredType = Vec<NodeDetails>;

    fn load(dir: PathBuf) -> JPreprocessResult<Self> {
        let dict = Dictionary::load(
            dir.join("jpreprocess.words"),
            dir.join("jpreprocess.wordsidx"),
        )?;
        Ok(Self(dict))
    }
    fn get(&self, index: usize) -> Option<Self::StoredType> {
        self.0
            .get(index)
            .and_then(|data| bincode::deserialize(data).ok())
    }
    fn iter(&self) -> DictionaryIter<Self::StoredType> {
        DictionaryIter::new(self)
    }
}