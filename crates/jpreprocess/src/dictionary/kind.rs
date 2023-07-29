use lindera_core::dictionary::Dictionary;

/// Specifies the kind of self-contained dictionary used for tokenization and preprocessing.
pub enum JPreprocessDictionaryKind {
    #[cfg(feature = "naist-jdic")]
    NaistJdic,
}

impl JPreprocessDictionaryKind {
    pub(crate) fn load(&self) -> Dictionary {
        match &self {
            #[cfg(feature = "naist-jdic")]
            Self::NaistJdic => jpreprocess_naist_jdic::lindera::load_dictionary().unwrap(),

            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
