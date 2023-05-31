mod dictionary;
mod normalize_text;

pub use dictionary::*;
use jpreprocess_core::{error::JPreprocessErrorKind, *};
use jpreprocess_dictionary::JPreprocessDictionary;
pub use jpreprocess_njd::NJD;
use lindera_tokenizer::tokenizer::Tokenizer;
pub use normalize_text::normalize_text_for_naist_jdic;

pub struct JPreprocess {
    tokenizer: Tokenizer,
    dictionary: Option<JPreprocessDictionary>,
}

impl JPreprocess {
    pub fn new(config: JPreprocessDictionaryConfig) -> JPreprocessResult<Self> {
        let (tokenizer, dictionary) = config.load()?;

        Ok(Self {
            tokenizer,
            dictionary,
        })
    }

    pub fn text_to_njd(&self, text: &str) -> JPreprocessResult<NJD> {
        let normalized_input_text = normalize_text_for_naist_jdic(text);
        let tokens = self
            .tokenizer
            .tokenize(normalized_input_text.as_str())
            .map_err(|err| JPreprocessErrorKind::LinderaError.with_error(err))?;

        if let Some(dictionary) = self.dictionary.as_ref() {
            NJD::from_tokens_dict(tokens, dictionary)
        } else {
            NJD::from_tokens_string(tokens)
        }
    }

    pub fn run_frontend(&self, text: &str) -> JPreprocessResult<Vec<String>> {
        let mut njd = Self::text_to_njd(&self, text)?;
        njd.proprocess();
        Ok(njd.into())
    }

    pub fn make_label(&self, njd_features: &[&str]) -> Vec<String> {
        let njd = NJD::from_strings(njd_features);
        jpreprocess_jpcommon::njdnodes_to_features(&njd.nodes)
    }

    pub fn extract_fullcontext(&self, text: &str) -> JPreprocessResult<Vec<String>> {
        let mut njd = Self::text_to_njd(&self, text)?;
        njd.proprocess();
        Ok(jpreprocess_jpcommon::njdnodes_to_features(&njd.nodes))
    }
}
