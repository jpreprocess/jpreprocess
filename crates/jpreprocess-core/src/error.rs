use crate::{
    accent_rule::AccentRuleParseError, cform::CFormParseError, ctype::CTypeParseError,
    pronunciation::PronunciationParseError, varint::UnexpectedEndOfBufferError,
    word_details::WordDetailsError,
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
    #[error("Failed to parse word entry: {0}")]
    WordEntryParseError(#[from] DictionaryCodecError),
}

#[derive(Debug, thiserror::Error)]
pub enum DictionaryError {
    #[error("Word with id {0} not found")]
    IdNotFound(u32),
    #[error("The word is flagged as UserDictionary, but Lindera UserDictionary is empty")]
    UserDictionaryNotProvided,
    #[error("The word is flagged as UserDictionary, but UserDictionary mode is not set")]
    UserDictionaryModeNotSet,
}

#[derive(Debug, thiserror::Error)]
pub enum DictionaryCodecError {
    #[error("Failed to parse pronunciation: {0}")]
    PronunciationParseError(#[from] PronunciationParseError),
    #[error("Invalid conjugation type (CType): {0}")]
    CTypeParseError(#[from] CTypeParseError),
    #[error("Failed to parse conjugation form (CForm): {0}")]
    CFormParseError(#[from] CFormParseError),
    #[error("Failed to parse accent rule: {0}")]
    AccentRuleParseError(#[from] AccentRuleParseError),
    #[error("Failed to parse word details: {0}")]
    WordDetailsParseError(#[from] WordDetailsError),
    #[error("Failed to parse word entry: {0}")]
    WordEntryParseError(UnexpectedEndOfBufferError),
}
