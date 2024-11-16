use jpreprocess_core::word_entry::WordEntry;

pub mod tokenizer;

pub trait Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> Vec<impl 'a + Token>;
}

pub trait Token {
    fn get_string(&mut self) -> &str;
    fn get_word_entry(&mut self) -> WordEntry;
}

impl Tokenizer for lindera::tokenizer::Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> Vec<impl 'a + Token> {
        self.tokenize(text).unwrap()
    }
}

impl Token for lindera::token::Token<'_> {
    fn get_string(&mut self) -> &str {
        &self.text
    }
    fn get_word_entry(&mut self) -> WordEntry {
        let details = self.details();
        WordEntry::load(&details).unwrap()
    }
}
