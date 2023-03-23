#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum JPreprocessErrorKind {
    Io,
    DictionaryIndexOutOfRange,
    DictionaryLoadError,
    DictionaryBuildError,
    LinderaError,
    PronounciationParseError,
    PartOfSpeechParseError,
    CTypeParseError,
    CFormParseError,
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
