use byteorder::{ByteOrder, LittleEndian};

use crate::WordDictionaryMode;

pub fn get_metadata(words_idx_data: &[u8], words_data: &[u8]) -> Option<String> {
    let metadata_end = LittleEndian::read_u32(&words_idx_data[0..4]) as usize;
    if metadata_end == 0 {
        return None;
    }

    String::from_utf8(words_data[0..metadata_end].to_vec()).ok()
}

pub fn detect_dictionary(words_idx_data: &[u8], words_data: &[u8]) -> WordDictionaryMode {
    if let Some(metadata) = get_metadata(words_idx_data, words_data) {
        if metadata.starts_with("JPreprocess") {
            return WordDictionaryMode::JPreprocess;
        }
    }
    WordDictionaryMode::Lindera
}
