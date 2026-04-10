pub mod accent_rule;
pub mod error;
pub mod pos;
pub mod pronunciation;
pub mod word_details;
pub mod word_entry;
pub mod word_line;

pub mod cform;
pub mod ctype;

pub mod token;

pub use error::JPreprocessError;
pub type JPreprocessResult<T> = Result<T, JPreprocessError>;
