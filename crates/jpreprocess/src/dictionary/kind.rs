use jpreprocess_dictionary::JPreprocessDictionary;

/// Specifies the kind of self-contained dictionary used for tokenization and preprocessing.
pub enum JPreprocessDictionaryKind {
    #[cfg(feature = "naist-jdic")]
    NaistJdic,
}

impl JPreprocessDictionaryKind {
    pub(crate) fn load(&self) -> (lindera_core::dictionary::Dictionary, JPreprocessDictionary) {
        match &self {
            #[cfg(feature = "naist-jdic")]
            Self::NaistJdic => (
                jpreprocess_naist_jdic::lindera::load_dictionary().unwrap(),
                jpreprocess_naist_jdic::jpreprocess::load_dictionary(),
            ),
            #[cfg(not(feature = "naist-jdic"))]
            _ => unreachable!(),
        }
    }
}
