use jpreprocess_dictionary::word_data::get_word_data;
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
    pub fn identifier(&self) -> Option<&str> {
        match self {
            Self::System(dict) => get_dict_preamble(&dict.words_idx_data, &dict.words_data),
            Self::User(dict) => get_dict_preamble(&dict.words_idx_data, &dict.words_data),
        }
    }
    pub fn get_bytes(&self, word_id: u32) -> Option<&[u8]> {
        match self {
            Self::System(dict) => get_word_data(
                &dict.words_idx_data,
                &dict.words_data,
                Some(word_id as usize),
            ),
            Self::User(dict) => get_word_data(
                &dict.words_idx_data,
                &dict.words_data,
                Some(word_id as usize),
            ),
        }
    }
}

fn get_dict_preamble<'a>(idx: &[u8], data: &'a [u8]) -> Option<&'a str> {
    match std::str::from_utf8(get_word_data(idx, data, None)?) {
        Ok(s) if s.is_empty() => {
            return None;
        }
        Ok(s) => {
            return Some(s);
        }
        Err(e) => {
            eprintln!("Error parsing dictionary type: {}", e);
            return None;
        }
    }
}
