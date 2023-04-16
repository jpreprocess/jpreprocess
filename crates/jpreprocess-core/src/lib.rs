pub mod accent_rule;
pub mod error;
pub mod word_details;
pub mod word_entry;
pub mod pos;
pub mod pronounciation;

pub mod cform;
pub mod ctype;

pub use error::JPreprocessError;
pub type JPreprocessResult<T> = Result<T, JPreprocessError>;
