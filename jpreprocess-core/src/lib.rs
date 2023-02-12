pub mod error;

pub use error::JPreprocessError;
pub type JPreprocessResult<T> = Result<T, JPreprocessError>;
