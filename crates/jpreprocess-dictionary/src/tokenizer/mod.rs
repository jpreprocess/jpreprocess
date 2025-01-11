use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};

pub mod default;

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
        let details = self.details();
        let entry = WordEntry::load(&details)?;
        Ok((&self.text, entry))
    }
}
