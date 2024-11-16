use std::borrow::Cow;

use jpreprocess_core::word_entry::WordEntry;

use super::{Token, Tokenizer};

pub struct WordDictionary {
    system: Vec<WordEntry>,
    user: Option<Vec<WordEntry>>,
}

impl WordDictionary {
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
    pub fn new(config: lindera::tokenizer::TokenizerConfig, word_dictionary: WordDictionary) -> Self {
        let tokenizer = lindera::tokenizer::Tokenizer::from_config(&config).unwrap();
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
