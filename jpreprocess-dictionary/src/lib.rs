use std::{
    error::Error,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use byteorder::{ByteOrder, LittleEndian};

pub struct Dictionary {
    words_data: Vec<u8>,
    words_idx_data: Vec<u32>,
}

impl Dictionary {
    pub fn load_lindera(dir: PathBuf) -> Result<Dictionary, Box<dyn Error>> {
        Self::load(dir, "dict")
    }
    pub fn load_jpreprocess(dir: PathBuf) -> Result<Dictionary, Box<dyn Error>> {
        Self::load(dir, "jpreprocess")
    }
    fn load(dir: PathBuf, name: &str) -> Result<Dictionary, Box<dyn Error>> {
        Ok(Self {
            words_data: Self::read_file(dir.join(format!("{}.words", name)))?,
            words_idx_data: Self::idx(dir.join(format!("{}.wordsidx", name)))?,
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

    fn read_file(path: PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(fs::read(path)?)
    }

    fn idx(path: PathBuf) -> Result<Vec<u32>, Box<dyn Error>> {
        let mut idx_file = File::open(path)?;
        let mut idxs = Vec::new();
        loop {
            let mut chunk = Vec::with_capacity(4);
            let n = idx_file.by_ref().take(4).read_to_end(&mut chunk)?;
            if n != 4 {
                break;
            }
            idxs.push(LittleEndian::read_u32(&chunk));
        }
        Ok(idxs)
    }

    pub fn iter(&self) -> DictionaryIter {
        DictionaryIter {
            dict: &self,
            index: 0,
        }
    }
}

pub struct DictionaryIter<'a> {
    dict: &'a Dictionary,
    index: usize,
}

impl<'a> Iterator for DictionaryIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.dict.get(self.index - 1)
    }
}
