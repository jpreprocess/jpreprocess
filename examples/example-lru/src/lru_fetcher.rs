use std::{
    fs::File,
    io::{Read, Seek},
    num::NonZeroUsize,
    path::Path,
    sync::Mutex,
};

use jpreprocess_core::{error::JPreprocessErrorKind, word_entry::WordEntry, JPreprocessResult};
use jpreprocess_dictionary::{default::WordDictionaryMode, DictionaryFetcher};
use lindera_tokenizer::token::Token;
use lru::LruCache;

pub struct LruFetcher {
    lru: Mutex<FileLru>,
}

impl LruFetcher {
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self, std::io::Error> {
        Ok(Self {
            lru: Mutex::new(FileLru::new(dir)?),
        })
    }
}

impl DictionaryFetcher for LruFetcher {
    fn get_word(&self, token: &Token) -> JPreprocessResult<WordEntry> {
        if token.word_id.is_unknown() {
            return Ok(WordEntry::default());
        }

        let mut g = self.lru.lock().unwrap();
        g.get_word(token.word_id.0)
    }
}

struct FileLru {
    mode: WordDictionaryMode,
    index: File,
    words: File,
    lru: LruCache<u32, WordEntry>,
}

impl FileLru {
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self, std::io::Error> {
        let mut index = File::open(dir.as_ref().join("dict.wordsidx"))?;
        let mut words = File::open(dir.as_ref().join("dict.words"))?;

        let mut index_buf = vec![0u8; 4];
        index.read_exact(&mut index_buf)?;
        let start = u32::from_be_bytes([index_buf[0], index_buf[1], index_buf[2], index_buf[3]]);

        let mut identifier_buf = vec![0u8; start as usize];
        words.read_exact(&mut identifier_buf)?;

        let mode = WordDictionaryMode::from_metadata(String::from_utf8(identifier_buf).ok());

        Ok(Self {
            mode,
            index,
            words,
            lru: LruCache::new(NonZeroUsize::new(1000).unwrap()),
        })
    }

    pub fn get_word(&mut self, index: u32) -> JPreprocessResult<WordEntry> {
        if let Some(word) = self.lru.get(&index) {
            dbg!("Found");
            return Ok(word.clone());
        }
        dbg!("Not Found");

        let bytes = self
            .get_bytes(index)
            .map_err(|err| JPreprocessErrorKind::Io.with_error(err))?;
        let entry = self.mode.into_serializer().deserialize(&bytes)?;
        self.lru.push(index, entry.clone());

        Ok(entry)
    }

    fn get_bytes(&mut self, index: u32) -> Result<Vec<u8>, std::io::Error> {
        let (start, end) = self.read_u32_range(index)?;

        self.words.seek(std::io::SeekFrom::Start(start as u64))?;
        if let Some(end) = end {
            let mut word_buf = vec![0u8; (end - start) as usize];
            self.words.read_exact(&mut word_buf)?;
            Ok(word_buf)
        } else {
            let mut word_buf = Vec::new();
            self.words.read_to_end(&mut word_buf)?;
            Ok(word_buf)
        }
    }
    fn read_u32_range(&mut self, index: u32) -> Result<(u32, Option<u32>), std::io::Error> {
        self.index
            .seek(std::io::SeekFrom::Start((index as u64) * 4))?;
        dbg!(index);
        let mut index_buf = vec![0u8; 8];
        if self.index.read_exact(&mut index_buf).is_ok() {
            let start =
                u32::from_le_bytes([index_buf[0], index_buf[1], index_buf[2], index_buf[3]]);
            let end = u32::from_le_bytes([index_buf[4], index_buf[5], index_buf[6], index_buf[7]]);
            return Ok((start, Some(end)));
        }

        self.index
            .seek(std::io::SeekFrom::Start((index as u64) * 4))?;
        let mut index_buf = vec![0u8; 4];
        self.index.read_exact(&mut index_buf)?;

        let start = u32::from_le_bytes([index_buf[0], index_buf[1], index_buf[2], index_buf[3]]);
        Ok((start, None))
    }
}
