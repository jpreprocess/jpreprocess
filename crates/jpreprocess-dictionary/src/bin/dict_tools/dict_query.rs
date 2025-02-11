use jpreprocess_dictionary::{
    dictionary::word_encoding::JPreprocessDictionaryWordEncoding, word_data::get_word_data,
};
use lindera::dictionary::{Dictionary, UserDictionary};
use lindera_dictionary::dictionary::{prefix_dictionary::PrefixDictionary, UNK};

pub enum QueryDict {
    System(Dictionary),
    User(UserDictionary),
}

impl QueryDict {
    pub fn dictionary_data(&self) -> &PrefixDictionary {
        match &self {
            Self::System(dict) => &dict.prefix_dictionary,
            Self::User(dict) => &dict.dict,
        }
    }
    pub fn identifier(&self) -> Option<&str> {
        match self {
            Self::System(dict) => get_dict_preamble(
                &dict.prefix_dictionary.words_idx_data,
                &dict.prefix_dictionary.words_data,
            ),
            Self::User(dict) => get_dict_preamble(&dict.dict.words_idx_data, &dict.dict.words_data),
        }
    }

    pub fn get_as_jpreprocess(
        &self,
        word_id: u32,
    ) -> Option<jpreprocess_core::word_entry::WordEntry> {
        let word_bin = match self {
            Self::System(dict) => get_word_data(
                &dict.prefix_dictionary.words_idx_data,
                &dict.prefix_dictionary.words_data,
                Some(word_id as usize),
            ),
            Self::User(dict) => get_word_data(
                &dict.dict.words_idx_data,
                &dict.dict.words_data,
                Some(word_id as usize),
            ),
        };

        JPreprocessDictionaryWordEncoding::deserialize(word_bin?).ok()
    }
    pub fn get_as_lindera(&self, word_id: u32) -> Option<Vec<&str>> {
        let result = match self {
            Self::System(dict) => dict.word_details(word_id as usize),
            Self::User(dict) => dict.word_details(word_id as usize),
        };

        if result == *UNK {
            None
        } else {
            Some(result)
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
