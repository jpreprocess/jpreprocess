use jpreprocess_core::JPreprocessResult;
use jpreprocess_dictionary::{default::WordDictionaryMode, DictionaryStore};
use lindera_core::{
    dictionary::{Dictionary, UserDictionary},
    prefix_dict::PrefixDict,
};

pub enum QueryDict {
    System(Dictionary),
    User(UserDictionary),
}

impl QueryDict {
    pub fn dictionary_data(&self) -> (&PrefixDict, &[u8], &[u8]) {
        match &self {
            Self::System(dict) => (&dict.dict, &dict.words_idx_data, &dict.words_data),
            Self::User(dict) => (&dict.dict, &dict.words_idx_data, &dict.words_data),
        }
    }
    pub fn mode(&self) -> WordDictionaryMode {
        match self {
            Self::System(dict) => WordDictionaryMode::from_metadata(dict.identifier()),
            Self::User(dict) => WordDictionaryMode::from_metadata(dict.identifier()),
        }
    }
}

impl<'a> DictionaryStore<'a> for QueryDict {
    fn get_bytes(&'a self, id: u32) -> JPreprocessResult<&'a [u8]> {
        match self {
            Self::System(dict) => dict.get_bytes(id),
            Self::User(dict) => dict.get_bytes(id),
        }
    }
    fn identifier(&self) -> Option<String> {
        match self {
            Self::System(dict) => dict.identifier(),
            Self::User(dict) => dict.identifier(),
        }
    }
}
