use jpreprocess_dictionary::{
    metadata::{detect_dictionary, get_metadata},
    query::DictionaryQuery,
    WordDictionaryMode,
};
use lindera_core::{
    dictionary::{Dictionary, UserDictionary},
    prefix_dict::PrefixDict,
    word_entry::WordId,
};

pub enum QueryDict {
    System(Dictionary),
    User(UserDictionary),
}

impl QueryDict {
    pub fn metadata(&self) -> Option<String> {
        match &self {
            Self::System(dict) => get_metadata(&dict.words_idx_data, &dict.words_data),
            Self::User(dict) => get_metadata(&dict.words_idx_data, &dict.words_data),
        }
    }
    pub fn mode(&self) -> WordDictionaryMode {
        match &self {
            Self::System(dict) => detect_dictionary(&dict.words_idx_data, &dict.words_data),
            Self::User(dict) => detect_dictionary(&dict.words_idx_data, &dict.words_data),
        }
    }
    fn dictionary(&self) -> &Dictionary {
        match &self {
            Self::System(dict) => dict,
            Self::User(_) => unreachable!(),
        }
    }
    fn user_dictionary(&self) -> &UserDictionary {
        match &self {
            Self::System(_) => unreachable!(),
            Self::User(dict) => dict,
        }
    }

    pub fn dictionary_data(&self) -> (&PrefixDict, &[u8], &[u8]) {
        match &self {
            Self::System(dict) => (&dict.dict, &dict.words_idx_data, &dict.words_data),
            Self::User(dict) => (&dict.dict, &dict.words_idx_data, &dict.words_data),
        }
    }
}

pub struct Query {
    pub word_id: u32,
    pub dict: QueryDict,
}
impl DictionaryQuery for Query {
    fn word_id(&self) -> WordId {
        WordId(
            self.word_id,
            match self.dict {
                QueryDict::System(_) => true,
                QueryDict::User(_) => false,
            },
        )
    }
    fn dictionary(&self) -> &Dictionary {
        self.dict.dictionary()
    }
    fn user_dictionary(&self) -> Option<&UserDictionary> {
        Some(self.dict.user_dictionary())
    }
}
