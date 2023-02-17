use std::{fs, path::PathBuf};

use byteorder::{ByteOrder, LittleEndian};

use jpreprocess_core::{error::JPreprocessErrorKind, JPreprocessResult};

pub struct Dictionary {
    words_data: Vec<u8>,
    words_idx_data: Vec<u32>,
}

impl Dictionary {
    pub fn load(words_path: PathBuf, idx_path: PathBuf) -> JPreprocessResult<Dictionary> {
        Ok(Self {
            words_data: Self::read_file(words_path)?,
            words_idx_data: Self::idx(idx_path)?,
        })
    }

    pub fn get(&self, index: usize) -> Option<&[u8]> {
        let curr = (*self.words_idx_data.get(index)?).try_into().ok()?;
        let next = match self.words_idx_data.get(index + 1) {
            Some(next) => (*next).try_into().ok()?,
            None => self.words_data.len(),
        };
        Some(&self.words_data[curr..next])
    }

    fn read_file(path: PathBuf) -> JPreprocessResult<Vec<u8>> {
        fs::read(path).map_err(|e| JPreprocessErrorKind::Io.with_error(e))
    }

    fn idx(path: PathBuf) -> JPreprocessResult<Vec<u32>> {
        Self::read_file(path).map(|idx| idx.chunks(4).map(LittleEndian::read_u32).collect())
    }
}

pub trait DictionaryTrait {
    type StoredType;

    fn load(dir: PathBuf) -> JPreprocessResult<Self>
    where
        Self: Sized;
    fn get(&self, index: usize) -> Option<Self::StoredType>;
    fn iter(&self) -> DictionaryIter<Self::StoredType>;
}

pub struct DictionaryIter<'a, T> {
    dict: &'a dyn DictionaryTrait<StoredType = T>,
    index: usize,
}

impl<'a, T> DictionaryIter<'a, T> {
    pub fn new<K>(dict: &'a K)->Self
    where
        K: DictionaryTrait<StoredType = T>,
    {
        Self { dict, index: 0 }
    }
}

impl<'a, T> Iterator for DictionaryIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.dict.get(self.index - 1)
    }
}
