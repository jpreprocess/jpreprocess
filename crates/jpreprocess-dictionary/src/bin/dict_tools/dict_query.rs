use jpreprocess_dictionary::{
    dictionary::word_encoding::JPreprocessDictionaryWordEncoding, word_data::PrefixDictionary,
};
use lindera_dictionary::dictionary::{Dictionary, UserDictionary, UNK};

pub enum QueryDict {
    System(Dictionary),
    User(UserDictionary),
}

impl QueryDict {
    pub fn identifier(&self) -> Option<String> {
        match self {
            Self::System(dict) => dict
                .prefix_dictionary
                .as_ref()
                .dict_ident()
                .map(String::from),
            Self::User(dict) => (&dict.dict).dict_ident().map(String::from),
        }
    }

    pub fn get_as_jpreprocess(
        &self,
        word_id: u32,
    ) -> Option<jpreprocess_core::word_entry::WordEntry> {
        fn get_word(
            prefix_dict: impl PrefixDictionary,
            word_id: u32,
        ) -> Option<jpreprocess_core::word_entry::WordEntry> {
            let word_bin = prefix_dict.word_data(word_id as usize);
            JPreprocessDictionaryWordEncoding::deserialize(word_bin?).ok()
        }

        match self {
            Self::System(dict) => get_word(&*dict.prefix_dictionary, word_id),
            Self::User(dict) => get_word(&dict.dict, word_id),
        }
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
