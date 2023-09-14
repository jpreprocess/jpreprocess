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
//!  let config = JPreprocessConfig {
//!      dictionary: SystemDictionaryConfig::File(path),
//!      user_dictionary: None,
//!  };
//! let jpreprocess = JPreprocess::from_config(config)?;
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
pub use normalize_text::normalize_text_for_naist_jdic;

pub use jpreprocess_core::error;
pub use jpreprocess_njd::NJD;

use jpreprocess_core::*;
use jpreprocess_dictionary::{fetcher::WordDictionaryConfig, DictionaryFetcher, DictionaryStore};
use lindera_core::dictionary::{Dictionary, UserDictionary};
use lindera_dictionary::{load_user_dictionary, UserDictionaryConfig};
use lindera_tokenizer::tokenizer::Tokenizer;

pub struct JPreprocessConfig {
    pub dictionary: SystemDictionaryConfig,
    pub user_dictionary: Option<UserDictionaryConfig>,
}

pub struct JPreprocess {
    tokenizer: Tokenizer,
    dictionary_config: Box<dyn DictionaryFetcher>,
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

        Ok(Self::new(dictionary, user_dictionary))
    }

    /// Creates JPreprocess from dictionaries.
    ///
    /// Note: `new` before v0.2.0 has moved to `from_config`
    pub fn new(dictionary: Dictionary, user_dictionary: Option<UserDictionary>) -> Self {
        let dictionary_config = WordDictionaryConfig {
            system: dictionary.serlializer_hint(),
            user: user_dictionary
                .as_ref()
                .map(DictionaryStore::serlializer_hint),
        };

        let tokenizer = Tokenizer::new(
            dictionary,
            user_dictionary,
            lindera_core::mode::Mode::Normal,
        );

        Self {
            tokenizer,
            dictionary_config: Box::new(dictionary_config),
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
    /// assert!(phoneme_vec[2].1.starts_with("/A:-3+1+7"));
    ///
    /// // fullcontext label
    /// let fullcontext = overwrapping_phonemes(phoneme_vec);
    ///
    /// assert!(fullcontext[2].starts_with("sil^n-i+h=o"));
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    pub fn text_to_njd(&self, text: &str) -> JPreprocessResult<NJD> {
        let normalized_input_text = normalize_text_for_naist_jdic(text);
        let tokens = self.tokenizer.tokenize(normalized_input_text.as_str())?;

        NJD::from_tokens(&tokens, self.dictionary_config.as_ref())
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
        njd.preprocess();
        Ok(jpreprocess_jpcommon::njdnodes_to_features(&njd.nodes))
    }
}
