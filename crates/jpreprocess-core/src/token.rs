use crate::{word_entry::WordEntry, JPreprocessResult};

pub trait Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>>;
}

pub trait Token {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)>;
}

#[cfg(feature = "lindera")]
impl Tokenizer for lindera::tokenizer::Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        Ok(self.tokenize(text)?)
    }
}

#[cfg(feature = "lindera")]
impl Token for lindera::token::Token<'_> {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)> {
        use lindera_dictionary::dictionary::UNK;

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
