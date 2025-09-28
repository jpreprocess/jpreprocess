use std::{
    borrow::Cow,
    fs::File,
    io::{Read, Seek},
    num::NonZeroUsize,
    path::Path,
    sync::Mutex,
};

use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};
use jpreprocess_dictionary::{
    dictionary::word_encoding::JPreprocessDictionaryWordEncoding,
    tokenizer::{Token, Tokenizer},
};
use lindera_dictionary::{dictionary::Dictionary, dictionary_loader::metadata::MetadataLoader};
use lru::LruCache;

pub struct LruTokenizer {
    tokenizer: lindera::tokenizer::Tokenizer,
    words: Mutex<CachedStorage>,
}
impl LruTokenizer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let dictionary = load_dictionary(path.as_ref());
        let tokenizer = lindera::tokenizer::Tokenizer::new(lindera::segmenter::Segmenter::new(
            lindera_dictionary::mode::Mode::Normal,
            dictionary,
            None,
        ));

        let storage = CachedStorage::new(path)?;

        Ok(Self {
            tokenizer,
            words: Mutex::new(storage),
        })
    }
}
impl Tokenizer for LruTokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        let mut tokens = Vec::new();
        let mut words = self.words.lock().unwrap();
        for token in self.tokenizer.tokenize(text)? {
            let entry = if token.word_id.is_unknown() {
                WordEntry::default()
            } else if token.word_id.is_system() {
                words.get_word(token.word_id.id)?
            } else {
                todo!("User dictionary support is not complete in this example")
            };

            tokens.push(LruToken {
                text: token.surface,
                entry,
            });
        }

        Ok(tokens)
    }
}

pub struct LruToken<'a> {
    text: Cow<'a, str>,
    entry: WordEntry,
}
impl Token for LruToken<'_> {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)> {
        Ok((&self.text, self.entry.clone()))
    }
}

fn load_dictionary(path: &Path) -> Dictionary {
    use lindera_dictionary::{
        dictionary::prefix_dictionary::PrefixDictionary,
        dictionary_loader::{
            character_definition::CharacterDefinitionLoader,
            connection_cost_matrix::ConnectionCostMatrixLoader,
            unknown_dictionary::UnknownDictionaryLoader,
        },
        util::read_file,
    };

    let metadata = MetadataLoader::load(path).unwrap();

    let da_data = read_file(path.join("dict.da").as_path()).unwrap();
    let vals_data = read_file(path.join("dict.vals").as_path()).unwrap();

    let prefix_dictionary = PrefixDictionary::load(
        da_data,
        vals_data,
        &[] as &'static [u8],
        &[] as &'static [u8],
        true,
    );

    Dictionary {
        metadata,
        prefix_dictionary,
        connection_cost_matrix: ConnectionCostMatrixLoader::load(path).unwrap(),
        character_definition: CharacterDefinitionLoader::load(path).unwrap(),
        unknown_dictionary: UnknownDictionaryLoader::load(path).unwrap(),
    }
}

struct CachedStorage {
    index_file: File,
    words_file: File,
    cache: LruCache<u32, WordEntry>,
}

impl CachedStorage {
    pub fn new<P: AsRef<Path>>(dir: P) -> Result<Self, std::io::Error> {
        let index = File::open(dir.as_ref().join("dict.wordsidx"))?;
        let words = File::open(dir.as_ref().join("dict.words"))?;

        Ok(Self {
            index_file: index,
            words_file: words,
            cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
        })
    }

    pub fn get_word(&mut self, index: u32) -> JPreprocessResult<WordEntry> {
        if let Some(word) = self.cache.get(&index) {
            println!("Word #{} found in cache", index);
            return Ok(word.clone());
        }
        println!("Word #{} not found in cache", index);

        let bytes = self.get_bytes(index)?;
        let entry: WordEntry = JPreprocessDictionaryWordEncoding::deserialize(&bytes).unwrap();
        self.cache.push(index, entry.clone());

        Ok(entry)
    }

    fn get_bytes(&mut self, index: u32) -> Result<Vec<u8>, std::io::Error> {
        let (start, end) = self.read_u32_range(index)?;

        self.words_file
            .seek(std::io::SeekFrom::Start(start as u64))?;
        if let Some(end) = end {
            let mut word_buf = vec![0u8; (end - start) as usize];
            self.words_file.read_exact(&mut word_buf)?;
            Ok(word_buf)
        } else {
            let mut word_buf = Vec::new();
            self.words_file.read_to_end(&mut word_buf)?;
            Ok(word_buf)
        }
    }
    fn read_u32_range(&mut self, index: u32) -> Result<(u32, Option<u32>), std::io::Error> {
        self.index_file
            .seek(std::io::SeekFrom::Start((index as u64) * 4))?;
        let mut index_buf = vec![0u8; 8];
        if self.index_file.read_exact(&mut index_buf).is_ok() {
            let start =
                u32::from_le_bytes([index_buf[0], index_buf[1], index_buf[2], index_buf[3]]);
            let end = u32::from_le_bytes([index_buf[4], index_buf[5], index_buf[6], index_buf[7]]);
            return Ok((start, Some(end)));
        }

        self.index_file
            .seek(std::io::SeekFrom::Start((index as u64) * 4))?;
        let mut index_buf = vec![0u8; 4];
        self.index_file.read_exact(&mut index_buf)?;

        let start = u32::from_le_bytes([index_buf[0], index_buf[1], index_buf[2], index_buf[3]]);
        Ok((start, None))
    }
}
