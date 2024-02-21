mod binding;
mod structs;

use pyo3::{exceptions::PyRuntimeError, prelude::*};

#[pymodule]
fn jpreprocess(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<binding::JPreprocessPyBinding>()?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("JPREPROCESS_VERSION", ::jpreprocess::VERSION)?;
    Ok(())
}

pub fn into_runtime_error<E: ToString>(err: E) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}
