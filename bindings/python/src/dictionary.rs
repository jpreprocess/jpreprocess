use std::path::PathBuf;

use jpreprocess_dictionary::dictionary::to_dict::JPreprocessDictionaryBuilder;
use lindera_dictionary::dictionary_builder::{ipadic::IpadicBuilder, DictionaryBuilder};
use pyo3::{exceptions::PyAssertionError, pyfunction, PyResult};

use crate::into_runtime_error;

#[pyfunction]
#[pyo3(signature = (input, output, user=None, serializer=None))]
pub fn build_dictionary(
    input: PathBuf,
    output: PathBuf,
    user: Option<bool>,
    serializer: Option<&str>,
) -> PyResult<()> {
    let builder: Box<dyn DictionaryBuilder> = match serializer {
        Some("lindera") => Box::new(IpadicBuilder {}),
        Some("jpreprocess") | None => Box::new(JPreprocessDictionaryBuilder {}),
        _ => {
            return Err(PyAssertionError::new_err(
                "serializer must be either `lindera` or `jpreprocess`.",
            ))
        }
    };
    let user = user.unwrap_or(false);

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
