use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};

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
        let entry = if self.word_id.is_unknown() {
            WordEntry::default()
        } else {
            let details = self.details();
            WordEntry::load(&details)?
        };
        Ok((&self.text, entry))
    }
}
