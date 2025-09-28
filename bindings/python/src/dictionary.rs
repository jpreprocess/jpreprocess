use std::path::PathBuf;

use jpreprocess_dictionary::dictionary::to_dict::JPreprocessDictionaryBuilder;
use lindera_dictionary::{dictionary::metadata::Metadata, dictionary_builder::DictionaryBuilder};
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
    let user = user.unwrap_or(false);

    match serializer {
        Some("lindera") => {
            let builder = DictionaryBuilder::new(Metadata::default());

            if user {
                builder
                    .build_user_dictionary(&input, &output)
                    .map_err(into_runtime_error)?;
            } else {
                builder
                    .build_dictionary(&input, &output)
                    .map_err(into_runtime_error)?;
            }
        }
        Some("jpreprocess") | None => {
            let builder = JPreprocessDictionaryBuilder::new();

            if user {
                builder
                    .build_user_dictionary(&input, &output)
                    .map_err(into_runtime_error)?;
            } else {
                builder
                    .build_dictionary(&input, &output)
                    .map_err(into_runtime_error)?;
            }
        }

        _ => {
            return Err(PyAssertionError::new_err(
                "serializer must be either 'lindera' or 'jpreprocess'",
            ));
        }
    }

    Ok(())
}
