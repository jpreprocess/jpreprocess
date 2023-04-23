use std::path::PathBuf;

mod normalize_text;

use jpreprocess_core::{error::JPreprocessErrorKind, *};
pub use jpreprocess_njd::NJD;
use lindera::dictionary::DictionaryConfig;
pub use lindera::{mode::Mode, tokenizer::*};
pub use normalize_text::normalize_text_for_naist_jdic;

pub fn preprocess_to_njd_string(
    input_text: &str,
    dictionary_path: PathBuf,
) -> JPreprocessResult<NJD> {
    let normalized_input_text = normalize_text_for_naist_jdic(input_text);

    let tokenizer = {
        let dictionary = DictionaryConfig {
            kind: None,
            path: Some(dictionary_path),
        };

        let config = TokenizerConfig {
            dictionary,
            user_dictionary: None,
            mode: Mode::Normal,
        };
        Tokenizer::from_config(config)
            .map_err(|err| JPreprocessErrorKind::LinderaError.with_error(err))?
    };

    let tokens = tokenizer
        .tokenize(normalized_input_text.as_str())
        .map_err(|err| JPreprocessErrorKind::LinderaError.with_error(err))?;

    let mut njd = NJD::from_tokens_string(tokens)?;

    jpreprocess_njd::proprocess_njd(&mut njd);

    Ok(njd)
}

#[cfg(feature = "naist-jdic")]
pub fn preprocess_to_njd_dictionary(
    input_text: &str,
    dictionary_path: PathBuf,
) -> JPreprocessResult<NJD> {
    let normalized_input_text = normalize_text_for_naist_jdic(input_text);

    let tokenizer = Tokenizer::new(
        jpreprocess_naist_jdic::lindera::load_dictionary().unwrap(),
        None,
        Mode::Normal,
    );

    let tokens = tokenizer
        .tokenize(normalized_input_text.as_str())
        .map_err(|err| JPreprocessErrorKind::LinderaError.with_error(err))?;

    let mut njd = NJD::from_tokens_dict(
        tokens,
        jpreprocess_naist_jdic::jpreprocess::load_dictionary(),
    )?;

    jpreprocess_njd::proprocess_njd(&mut njd);

    Ok(njd)
}
