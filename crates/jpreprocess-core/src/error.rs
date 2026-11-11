use crate::{
    accent_rule::AccentRuleParseError, ctype::CTypeParseError,
    pronunciation::PronunciationParseError,
};

#[derive(Debug, thiserror::Error)]
pub enum JPreprocessError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to fetch word from dictionary: {0}")]
    DictionaryError(#[from] DictionaryError),
    #[cfg(feature = "lindera")]
    #[error("Lindera error: {0}")]
    LinderaError(#[from] lindera::error::LinderaError),
    #[error("Failed to parse pronunciation: {0}")]
    PronunciationParseError(#[from] PronunciationParseError),
    #[error("Failed to parse conjugation type (CType): {0}")]
    CTypeParseError(#[from] CTypeParseError),
    #[error("Failed to parse conjugation form (CForm)")]
    CFormParseError,
    #[error("Failed to parse accent rule: {0}")]
    AccentRuleParseError(#[from] AccentRuleParseError),
    #[error("Provided mora size {0} is different from that of calculated from pronunciation {1}")]
    MoraSizeMismatch(usize, usize),
}

#[derive(Debug, thiserror::Error)]
pub enum DictionaryError {
    #[error("Word with id {0} not found")]
    IdNotFound(u32),
    #[error("The word is flagged as UserDictionary, but Lindera UserDictionary is empty")]
    UserDictionaryNotProvided,
    #[error("The word is flagged as UserDictionary, but UserDictionary mode is not set")]
    UserDictionaryModeNotSet,
    #[error("Failed to parse binary dictionary: {0}")]
    BinaryParseError(#[from] BinaryParseError),
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse binary dictionary: {kind} in component {component:?}")]
pub struct BinaryParseError {
    #[source]
    pub kind: BinaryParseErrorKind,
    pub component: ParserComponent,
}

#[derive(Debug, thiserror::Error)]
pub enum BinaryParseErrorKind {
    #[error("Unexpected end of buffer while reading byte")]
    UnexpectedEndOfBuffer,
}

#[derive(Debug)]
pub enum ParserComponent {
    POS,
    CType,
    CForm,
    Read,
    Pronunciation,
    AccMoraSize,
    ChainRule,
    ChainFlag,
}
