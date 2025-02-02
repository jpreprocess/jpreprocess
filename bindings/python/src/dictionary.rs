use std::path::PathBuf;

use jpreprocess_dictionary::{
    dictionary::to_dict::ipadic_builder::IpadicBuilder,
    serializer::{
        jpreprocess::JPreprocessSerializer, lindera::LinderaSerializer, DictionarySerializer,
    },
};
use lindera_core::dictionary_builder::DictionaryBuilder;

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
    let serializer: Box<dyn DictionarySerializer + Send + Sync + 'static> = match serializer {
        Some("lindera") => Box::new(LinderaSerializer),
        Some("jpreprocess") | None => Box::new(JPreprocessSerializer),
        _ => {
            return Err(PyAssertionError::new_err(
                "serializer must be either `lindera` or `jpreprocess`.",
            ))
        }
    };
    let user = user.unwrap_or(false);

    let builder = IpadicBuilder::new(serializer);

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
