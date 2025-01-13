use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};

use super::{
    identify_dictionary::DictionaryIdent, jpreprocess::JPreprocessTokenizer, Token, Tokenizer,
};

pub enum DefaultTokenizer {
    JPreprocessTokenizer(JPreprocessTokenizer),
    LinderaTokenizer(lindera::tokenizer::Tokenizer),
}

impl DefaultTokenizer {
    pub fn new(tokenizer: lindera::tokenizer::Tokenizer) -> Self {
        let ident = DictionaryIdent::from_idx_data(
            &tokenizer
                .segmenter
                .dictionary
                .prefix_dictionary
                .words_idx_data,
            &tokenizer.segmenter.dictionary.prefix_dictionary.words_data,
        );
        match ident {
            DictionaryIdent::JPreprocess => {
                DefaultTokenizer::JPreprocessTokenizer(JPreprocessTokenizer::new(tokenizer))
            }
            DictionaryIdent::Lindera => DefaultTokenizer::LinderaTokenizer(tokenizer),
        }
    }
}

impl Tokenizer for DefaultTokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        match self {
            DefaultTokenizer::JPreprocessTokenizer(tokenizer) => Ok(tokenizer
                .tokenize(text)?
                .into_iter()
                .map(DefaultToken::from_token)
                .collect()),
            DefaultTokenizer::LinderaTokenizer(tokenizer) => Ok(tokenizer
                .tokenize(text)?
                .into_iter()
                .map(DefaultToken::from_token)
                .collect()),
        }
    }
}

struct DefaultToken<'a> {
    inner: Box<dyn 'a + Token>,
}

impl<'a> DefaultToken<'a> {
    fn from_token(inner: impl 'a + Token) -> Self {
        DefaultToken {
            inner: Box::new(inner),
        }
    }
}

impl<'a> Token for DefaultToken<'a> {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)> {
        self.inner.fetch()
    }
}
