use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};
use lindera_dictionary::dictionary::UNK;

pub mod default;
mod identify_dictionary;
pub mod jpreprocess;

pub trait Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>>;
}

pub trait Token {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)>;
}

impl Tokenizer for lindera::tokenizer::Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        Ok(self.tokenize(text)?)
    }
}

impl Token for lindera::token::Token<'_> {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)> {
        let mut details = self.details();
        let entry = if details == *UNK {
            WordEntry::default()
        } else {
            details.resize(12, "");
            WordEntry::load(&details)?
        };

        Ok((&self.surface, entry))
    }
}
