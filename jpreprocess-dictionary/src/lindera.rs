use std::path::PathBuf;

use jpreprocess_core::{JPreprocessResult};

use crate::{Dictionary, DictionaryTrait, DictionaryIter};


pub struct LinderaDict(Dictionary);
impl DictionaryTrait for LinderaDict {
    type StoredType = Vec<String>;

    fn load(dir: PathBuf) -> JPreprocessResult<Self> {
        let dict = Dictionary::load(dir.join("dict.words"), dir.join("dict.wordsidx"))?;
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
