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
//! #     let path = PathBuf::from("tests/min-dict");
//! let config = JPreprocessConfig {
//!      dictionary: SystemDictionaryConfig::File(path),
//!      user_dictionary: None,
//!  };
//! let jpreprocess = JPreprocess::from_config(config)?;
//!
//! let jpcommon_label = jpreprocess
//!     .extract_fullcontext("日本語文を解析し、音声合成エンジンに渡せる形式に変換します．")?;
//! assert_eq!(
//!   jpcommon_label[2].to_string(),
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

#[doc(hidden)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod dictionary;
mod normalize_text;

pub use dictionary::*;
pub use normalize_text::normalize_text_for_naist_jdic;

pub use jpreprocess_core::error;
pub use jpreprocess_dictionary::default::DefaultFetcher;
pub use jpreprocess_njd::NJD;
pub use lindera_core::dictionary::{Dictionary, UserDictionary};
pub use lindera_dictionary::{load_user_dictionary, DictionaryKind, UserDictionaryConfig};

use jpreprocess_core::*;
use jpreprocess_dictionary::DictionaryFetcher;
use lindera_tokenizer::tokenizer::Tokenizer;

pub struct JPreprocessConfig {
    pub dictionary: SystemDictionaryConfig,
    pub user_dictionary: Option<UserDictionaryConfig>,
}

pub struct JPreprocess<F: DictionaryFetcher> {
    tokenizer: Tokenizer,
    dictionary_fetcher: F,
}

impl JPreprocess<DefaultFetcher> {
    /// Loads the dictionary from JPreprocessConfig.
    ///
    /// This supports importing files and built-in dictionary (needs feature).
    /// If you need to import from data, please use [`with_dictionaries`] instead.
    ///
    /// [`with_dictionaries`]: #method.with_dictionaries
    ///
    /// ## Example 1: Load from file
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use std::path::PathBuf;
    /// use jpreprocess::*;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #     let path = PathBuf::from("tests/min-dict");
    ///  let config = JPreprocessConfig {
    ///      dictionary: SystemDictionaryConfig::File(path),
    ///      user_dictionary: None,
    ///  };
    /// let jpreprocess = JPreprocess::from_config(config)?;
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
    ///  let config = JPreprocessConfig {
    ///      dictionary: SystemDictionaryConfig::Bundled(JPreprocessDictionaryKind::NaistJdic),
    ///      user_dictionary: None,
    ///  };
    /// let jpreprocess = JPreprocess::from_config(config)?;
    /// #
    /// #     Ok(())
    /// # }
    /// # #[cfg(not(feature = "naist-jdic"))]
    /// # fn main() {}
    /// ```
    pub fn from_config(config: JPreprocessConfig) -> JPreprocessResult<Self> {
        let dictionary = config.dictionary.load()?;

        let user_dictionary = match config.user_dictionary {
            Some(user_dict_conf) => Some(load_user_dictionary(user_dict_conf)?),
            None => None,
        };

        Ok(Self::with_dictionaries(dictionary, user_dictionary))
    }

    /// Creates JPreprocess with provided dictionary data.
    pub fn with_dictionaries(
        dictionary: Dictionary,
        user_dictionary: Option<UserDictionary>,
    ) -> Self {
        let dictionary_fetcher =
            DefaultFetcher::from_dictionaries(&dictionary, user_dictionary.as_ref());

        Self::with_dictionary_fetcher(dictionary_fetcher, dictionary, user_dictionary)
    }
}

impl<F: DictionaryFetcher> JPreprocess<F> {
    /// Creates JPreprocess with provided dictionary fetcher.
    pub fn with_dictionary_fetcher(
        dictionary_fetcher: F,
        dictionary: Dictionary,
        user_dictionary: Option<UserDictionary>,
    ) -> Self {
        let tokenizer = Tokenizer::new(
            dictionary,
            user_dictionary,
            lindera_core::mode::Mode::Normal,
        );

        Self {
            tokenizer,
            dictionary_fetcher,
        }
    }

    /// Tokenize input text and return NJD.
    ///
    /// Useful for customizing text processing.
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use std::path::PathBuf;
    /// use jpreprocess::*;
    /// use jpreprocess_jpcommon::*;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #     let path = PathBuf::from("tests/min-dict");
    /// #  let config = JPreprocessConfig {
    /// #      dictionary: SystemDictionaryConfig::File(path),
    /// #      user_dictionary: None,
    /// #  };
    /// let jpreprocess = JPreprocess::from_config(config)?;
    ///
    /// let mut njd = jpreprocess.text_to_njd("日本語文を解析し、音声合成エンジンに渡せる形式に変換します．")?;
    /// njd.preprocess();
    ///
    /// // jpcommon utterance
    /// let utterance = Utterance::from(njd.nodes.as_slice());
    ///
    /// // Vec<([phoneme string], [context labels])>
    /// let phoneme_vec = utterance_to_phoneme_vec(&utterance);
    ///
    /// assert_eq!(&phoneme_vec[2].0, "i");
    ///
    /// // fullcontext label
    /// let fullcontext = overwrapping_phonemes(phoneme_vec);
    ///
    /// assert!(fullcontext[2].to_string().starts_with("sil^n-i+h=o"));
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text_to_njd(&self, text: &str) -> JPreprocessResult<NJD> {
        let normalized_input_text = normalize_text_for_naist_jdic(text);
        let tokens = self.tokenizer.tokenize(normalized_input_text.as_str())?;

        NJD::from_tokens(&tokens, &self.dictionary_fetcher)
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
        njd.preprocess();
        Ok(njd.into())
    }

    /// Generate jpcommon features from NJD features(returned by [`run_frontend`]).
    ///
    /// [`run_frontend`]: #method.run_frontend
    pub fn make_label(&self, njd_features: Vec<String>) -> Vec<jlabel::Label> {
        let njd = NJD::from_strings(njd_features);
        jpreprocess_jpcommon::njdnodes_to_features(&njd.nodes)
    }

    /// Generate jpcommon features from a text.
    ///
    /// This is not guaranteed to be same as calling [`run_frontend`] and [`make_label`].
    ///
    /// [`run_frontend`]: #method.run_frontend
    /// [`make_label`]: #method.make_label
    pub fn extract_fullcontext(&self, text: &str) -> JPreprocessResult<Vec<jlabel::Label>> {
        let mut njd = Self::text_to_njd(self, text)?;
        njd.preprocess();
        Ok(jpreprocess_jpcommon::njdnodes_to_features(&njd.nodes))
    }
}

#[cfg(test)]
mod tests {
    use jpreprocess_dictionary::default::DefaultFetcher;

    use crate::JPreprocess;

    #[test]
    fn multithread() {
        fn tester<T: Send + Sync>() {}
        tester::<JPreprocess<DefaultFetcher>>();
    }
}
