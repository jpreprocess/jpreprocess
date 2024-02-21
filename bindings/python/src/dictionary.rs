use std::path::PathBuf;

use jpreprocess_dictionary::default::WordDictionaryMode;
use lindera_core::dictionary_builder::DictionaryBuilder;

use jpreprocess_dictionary_builder::ipadic_builder::IpadicBuilder;
use pyo3::{exceptions::PyAssertionError, pyfunction, PyResult};

use crate::into_runtime_error;

#[pyfunction]
pub fn build_dictionary(
    input: PathBuf,
    output: PathBuf,
    user: Option<bool>,
    serializer: Option<&str>,
) -> PyResult<()> {
    let serializer = match serializer {
        Some("lindera") => WordDictionaryMode::Lindera,
        Some("jpreprocess") | None => WordDictionaryMode::JPreprocess,
        _ => {
            return Err(PyAssertionError::new_err(
                "serializer must be either `lindera` or `jpreprocess`.",
            ))
        }
    };
    let user = user.unwrap_or(false);

    let builder = IpadicBuilder::new(serializer.into_serializer());

    if user {
        builder
            .build_user_dictionary(&input, &output)
            .map_err(into_runtime_error)?;
    } else {
        builder
            .build_dictionary(&input, &output)
            .map_err(into_runtime_error)?;
    }

    Ok(())
}
