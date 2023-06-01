use jpreprocess_dictionary::{Dictionary, JPreprocessDictionary};

#[cfg(feature = "naist-jdic")]
const WORDS_IDX_DATA: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/jpreprocess.wordsidx"));
#[cfg(not(feature = "naist-jdic"))]
const WORDS_IDX_DATA: &[u8] = &[];

#[cfg(feature = "naist-jdic")]
const WORDS_DATA: &'static [u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/naist-jdic/jpreprocess.words"));
#[cfg(not(feature = "naist-jdic"))]
const WORDS_DATA: &[u8] = &[];

pub fn load_dictionary() -> JPreprocessDictionary {
    Dictionary::load_bin(words_data(), words_idx_data()).into()
}

pub fn words_idx_data() -> Vec<u8> {
    WORDS_IDX_DATA.to_vec()
}

pub fn words_data() -> Vec<u8> {
    WORDS_DATA.to_vec()
}
