use jpreprocess_dictionary::{
    metadata::{detect_dictionary, get_metadata},
    query::DictionaryQuery,
    WordDictionaryMode,
};
use lindera_core::{
    dictionary::{Dictionary, UserDictionary},
    word_entry::WordId,
};

pub enum QueryDict {
    System(Dictionary),
    User(UserDictionary),
}

impl QueryDict {
    pub fn metadata(&self) -> Option<String> {
        match &self {
            QueryDict::System(dict) => get_metadata(&dict.words_idx_data, &dict.words_data),
            QueryDict::User(dict) => get_metadata(&dict.words_idx_data, &dict.words_data),
        }
    }
    pub fn mode(&self) -> WordDictionaryMode {
        match &self {
            QueryDict::System(dict) => detect_dictionary(&dict.words_idx_data, &dict.words_data),
            QueryDict::User(dict) => detect_dictionary(&dict.words_idx_data, &dict.words_data),
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
        match &self.dict {
            QueryDict::System(dict) => dict,
            QueryDict::User(_) => unreachable!(),
        }
    }
    fn user_dictionary(&self) -> Option<&UserDictionary> {
        match &self.dict {
            QueryDict::System(_) => unreachable!(),
            QueryDict::User(dict) => Some(dict),
        }
    }
}
