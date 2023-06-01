//! Japanese text preprocessor for Text-to-Speech application (OpenJTalk rewrite in rust language).
//!
//! ## Example
//!
//! ```rust
//! # use std::error::Error;
//! # use std::path::PathBuf;
//! use jpreprocess::*;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! #     let path = PathBuf::from("tests/dict");
//! let config = JPreprocessDictionaryConfig::FileLindera(path);
//! let jpreprocess = JPreprocess::new(config)?;
//!
//! let jpcommon_label = jpreprocess
//!     .extract_fullcontext("日本語文を解析し、音声合成エンジンに渡せる形式に変換します．")?;
//! assert_eq!(
//!   jpcommon_label[2],
//!   concat!(
//!       "sil^n-i+h=o",
//!       "/A:-3+1+7",
//!       "/B:xx-xx_xx",
//!       "/C:02_xx+xx",
//!       "/D:02+xx_xx",
//!       "/E:xx_xx!xx_xx-xx",
//!       "/F:7_4#0_xx@1_3|1_12",
//!       "/G:4_4%0_xx_1",
//!       "/H:xx_xx",
//!       "/I:3-12@1+2&1-8|1+41",
//!       "/J:5_29",
//!       "/K:2+8-41"
//!   )
//! );
//! #
//! #     Ok(())
//! # }
//! ```

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
    /// Loads the dictionary.
    ///
    /// ## Example 1: Load from file
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use std::path::PathBuf;
    /// use jpreprocess::*;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #     let path = PathBuf::from("tests/dict");
    /// let config = JPreprocessDictionaryConfig::FileLindera(path);
    /// let jpreprocess = JPreprocess::new(config)?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// ## Example 2: Load bundled dictionary (This requires a feature to be enabled)
    ///
    /// ```rust
    /// # use std::error::Error;
    /// use jpreprocess::{*, kind::*};
    ///
    /// # #[cfg(feature = "naist-jdic")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let config = JPreprocessDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic);
    /// let jpreprocess = JPreprocess::new(config)?;
    /// #
    /// #     Ok(())
    /// # }
    /// # #[cfg(not(feature = "naist-jdic"))]
    /// # fn main() {}
    /// ```
    pub fn new(config: JPreprocessDictionaryConfig) -> JPreprocessResult<Self> {
        let (tokenizer, dictionary) = config.load()?;

        Ok(Self {
            tokenizer,
            dictionary,
        })
    }

    /// Tokenize a text and return NJD.
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

    /// Tokenize a text, preprocess, and return NJD converted to string.
    ///
    /// The returned string does not match that of openjtalk.
    /// JPreprocess drops orig string and some of the CForm information,
    /// which is unnecessary to preprocessing.
    ///
    /// If you need these infomation, please raise a feature request as an issue.
    pub fn run_frontend(&self, text: &str) -> JPreprocessResult<Vec<String>> {
        let mut njd = Self::text_to_njd(self, text)?;
        njd.proprocess();
        Ok(njd.into())
    }

    /// Generate jpcommon features from NJD features(returned by [`run_frontend`]).
    ///
    /// [`run_frontend`]: #method.run_frontend
    pub fn make_label(&self, njd_features: Vec<String>) -> Vec<String> {
        let njd = NJD::from_strings(njd_features);
        jpreprocess_jpcommon::njdnodes_to_features(&njd.nodes)
    }

    /// Generate jpcommon features from a text.
    ///
    /// This is not guaranteed to be same as calling [`run_frontend`] and [`make_label`].
    ///
    /// [`run_frontend`]: #method.run_frontend
    /// [`make_label`]: #method.make_label
    pub fn extract_fullcontext(&self, text: &str) -> JPreprocessResult<Vec<String>> {
        let mut njd = Self::text_to_njd(self, text)?;
        njd.proprocess();
        Ok(jpreprocess_jpcommon::njdnodes_to_features(&njd.nodes))
    }
}
