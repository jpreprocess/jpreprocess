use std::path::PathBuf;

use jpreprocess_core::{word_details::WordDetails, JPreprocessResult};

use crate::{Dictionary, DictionaryIter, DictionaryTrait};

pub struct JPreprocessDictionary(Dictionary);
impl DictionaryTrait for JPreprocessDictionary {
    type StoredType = Vec<WordDetails>;

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
impl From<Dictionary> for JPreprocessDictionary {
    fn from(dict: Dictionary) -> Self {
        Self(dict)
    }
}
