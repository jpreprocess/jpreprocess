use lindera_core::error::LinderaError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum JPreprocessErrorKind {
    Io,
    WordNotFoundError,
    DictionaryLoadError,
    LinderaError,
    PronunciationParseError,
    PartOfSpeechParseError,
    CTypeParseError,
    CFormParseError,
    AccentRuleParseError,
}

impl JPreprocessErrorKind {
    pub fn with_error<E>(self, source: E) -> JPreprocessError
    where
        anyhow::Error: From<E>,
    {
        JPreprocessError {
            kind: self,
            source: From::from(source),
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[error("JPreprocessError(kind={kind:?}, source={source})")]
pub struct JPreprocessError {
    pub kind: JPreprocessErrorKind,
    source: anyhow::Error,
}

impl From<LinderaError> for JPreprocessError {
    fn from(value: LinderaError) -> Self {
        JPreprocessErrorKind::LinderaError.with_error(value)
    }
}
