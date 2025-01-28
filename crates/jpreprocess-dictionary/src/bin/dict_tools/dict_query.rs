use lindera::dictionary::{Dictionary, UserDictionary};
use lindera_dictionary::dictionary::prefix_dictionary::PrefixDictionary;

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
    pub fn get_bytes(&self, word_id: u32) -> Option<&[u8]> {
        match self {
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
            log::warn!("Error parsing dictionary type: {}", e);
            return None;
        }
    }
}

fn get_word_data<'a>(idx: &[u8], data: &'a [u8], word_id: Option<usize>) -> Option<&'a [u8]> {
    let get_idx = |word_id: usize| -> Option<usize> {
        if word_id * 4 + 4 > idx.len() {
            return None;
        }
        Some(u32::from_le_bytes([
            idx[word_id * 4],
            idx[word_id * 4 + 1],
            idx[word_id * 4 + 2],
            idx[word_id * 4 + 3],
        ]) as usize)
    };

    let start = get_idx(word_id.unwrap_or(0))?;
    let end = get_idx(word_id.unwrap_or(0) + 1).unwrap_or(data.len());

    let range = if word_id.is_some() {
        start..end
    } else {
        0..start
    };

    if range.end < data.len() {
        Some(&data[range])
    } else {
        None
    }
}
