use std::borrow::Cow;

use jpreprocess_core::word_entry::WordEntry;

use super::{Token, Tokenizer};

pub struct WordDictionary {
    system: Vec<WordEntry>,
    user: Option<Vec<WordEntry>>,
}

impl WordDictionary {
    fn from_linderadict(
        system: &lindera_dictionary::dictionary::Dictionary,
        user: Option<&lindera_dictionary::dictionary::UserDictionary>,
    ) -> Self {
        fn parse_words(words_idx_data: &[u8], words_data: &[u8]) -> Vec<WordEntry> {
            let idx_vec = words_idx_data
                .chunks(4)
                .map(|chunk| {
                    assert!(chunk.len() == 4);
                    u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
                })
                .collect::<Vec<_>>();
            let mut parsed = Vec::with_capacity(idx_vec.len());
            for i in 0..idx_vec.len() {
                let start = idx_vec[i] as usize;
                let end = if i + 1 < idx_vec.len() {
                    idx_vec[i + 1] as usize
                } else {
                    words_data.len()
                };
                let entry = bincode::deserialize(&words_data[start..end]).unwrap();
                parsed.push(entry);
            }
            parsed
        }
        Self {
            system: parse_words(
                &system.prefix_dictionary.words_idx_data,
                &system.prefix_dictionary.words_data,
            ),
            user: user.map(|user| parse_words(&user.dict.words_idx_data, &user.dict.words_data)),
        }
    }
    fn get(&self, word_id: lindera::dictionary::WordId) -> WordEntry {
        let word = if word_id.is_unknown() {
            None
        } else if word_id.is_system() {
            self.system.get(word_id.id as usize)
        } else {
            self.user
                .as_ref()
                .and_then(|user| user.get(word_id.id as usize))
        };
        word.cloned().unwrap_or_default()
    }
}

pub struct DefaultTokenizer {
    tokenizer: lindera::tokenizer::Tokenizer,
    word_dictionary: WordDictionary,
}

impl DefaultTokenizer {
    pub fn new(segmenter: lindera::segmenter::Segmenter) -> Self {
        let word_dictionary = WordDictionary::from_linderadict(
            &segmenter.dictionary,
            segmenter.user_dictionary.as_ref(),
        );
        let tokenizer = lindera::tokenizer::Tokenizer::new(segmenter);
        Self {
            tokenizer,
            word_dictionary,
        }
    }
}

impl Tokenizer for DefaultTokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> Vec<impl 'a + Token> {
        let words = self.tokenizer.tokenize(text).unwrap();
        words
            .into_iter()
            .map(|token| JPreprocessToken {
                text: token.text,
                entry: self.word_dictionary.get(token.word_id),
            })
            .collect()
    }
}

pub struct JPreprocessToken<'a> {
    text: Cow<'a, str>,
    entry: WordEntry,
}

impl Token for JPreprocessToken<'_> {
    fn get_string(&mut self) -> &str {
        &self.text
    }
    fn get_word_entry(&mut self) -> WordEntry {
        self.entry.clone()
    }
}
