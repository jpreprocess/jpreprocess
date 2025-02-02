use jpreprocess_core::{word_entry::WordEntry, JPreprocessResult};

use crate::tokenizer::PrefixDictionary;

use super::{
    identify_dictionary::DictionaryIdent,
    jpreprocess::{JPreprocessToken, JPreprocessTokenizer},
    Token, Tokenizer,
};

pub struct DefaultTokenizer {
    lindera_tokenizer: lindera_tokenizer::tokenizer::Tokenizer,
    system: TokenizerType,
    user: Option<TokenizerType>,
}

enum TokenizerType {
    JPreprocessTokenizer,
    LinderaTokenizer,
}

impl DefaultTokenizer {
    pub fn new(tokenizer: lindera_tokenizer::tokenizer::Tokenizer) -> Self {
        fn identify_tokenizer(prefix_dictionary: &PrefixDictionary) -> TokenizerType {
            let ident = DictionaryIdent::from_idx_data(
                prefix_dictionary.words_idx_data,
                prefix_dictionary.words_data,
            );
            match ident {
                DictionaryIdent::JPreprocess => TokenizerType::JPreprocessTokenizer,
                DictionaryIdent::Lindera => TokenizerType::LinderaTokenizer,
            }
        }

        Self {
            system: identify_tokenizer(&PrefixDictionary::from_dictionary(&tokenizer.dictionary)),
            user: tokenizer
                .user_dictionary
                .as_ref()
                .map(|d| identify_tokenizer(&PrefixDictionary::from_user_dictionary(d))),
            lindera_tokenizer: tokenizer,
        }
    }
}

impl Tokenizer for DefaultTokenizer {
    fn tokenize<'a>(&'a self, text: &'a str) -> JPreprocessResult<Vec<impl 'a + Token>> {
        let tokens = self.lindera_tokenizer.tokenize(text)?;

        tokens
            .into_iter()
            .map(|token| {
                if token.word_id.is_unknown() {
                    Ok(DefaultToken::from_token(token))
                } else if token.word_id.is_system() {
                    match self.system {
                        TokenizerType::JPreprocessTokenizer => {
                            Ok(DefaultToken::from_token(JPreprocessToken::new(
                                token.text,
                                JPreprocessTokenizer::get_word_from_prefixdict(
                                    &PrefixDictionary::from_dictionary(token.dictionary),
                                    token.word_id,
                                )?,
                            )))
                        }
                        TokenizerType::LinderaTokenizer => Ok(DefaultToken::from_token(token)),
                    }
                } else {
                    match self.user {
                        Some(TokenizerType::JPreprocessTokenizer) => {
                            Ok(DefaultToken::from_token(JPreprocessToken::new(
                                token.text,
                                JPreprocessTokenizer::get_word_from_prefixdict(
                                    &PrefixDictionary::from_user_dictionary(
                                        token.user_dictionary.as_ref().unwrap(),
                                    ),
                                    token.word_id,
                                )?,
                            )))
                        }
                        Some(TokenizerType::LinderaTokenizer) => {
                            Ok(DefaultToken::from_token(token))
                        }
                        None => Ok(DefaultToken::from_token(token)),
                    }
                }
            })
            .collect()
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

impl Token for DefaultToken<'_> {
    fn fetch(&mut self) -> JPreprocessResult<(&str, WordEntry)> {
        self.inner.fetch()
    }
}
