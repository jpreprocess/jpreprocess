use crate::{word_entry::WordEntry, JPreprocessResult};

pub trait Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>>;
}

pub trait Token {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)>;
}

impl Token for (String, WordEntry) {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)> {
        let (string, entry) = self;
        Ok((string.as_str(), entry.to_owned()))
    }
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

/// Vibrato support is experimental and may be removed or changed in the future. Use with caution.
#[cfg(feature = "vibrato")]
impl Tokenizer for vibrato::tokenizer::Tokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        let mut worker = self.new_worker();
        worker.reset_sentence(text);
        worker.tokenize();

        worker
            .token_iter()
            .map(|token| {
                let features = token.feature();
                let mut details = features.split(',').collect::<Vec<_>>();
                details.resize(12, "*");
                let entry = WordEntry::load(&details)?;

                Ok((token.surface().to_string(), entry))
            })
            .collect()
    }
}
