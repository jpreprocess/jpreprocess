use crate::word_data::get_word_data;

pub enum DictionaryIdent {
    Lindera,
    JPreprocess,
}

impl DictionaryIdent {
    pub fn from_idx_data(idx: &[u8], data: &[u8]) -> Self {
        let Some(data) = get_word_data(idx, data, None) else {
            return DictionaryIdent::Lindera;
        };

        match std::str::from_utf8(data) {
            Ok("") => DictionaryIdent::Lindera,
            Ok(ident) if ident.to_lowercase().starts_with("jpreprocess") => {
                DictionaryIdent::JPreprocess
            }
            Err(e) => {
                log::warn!("Error parsing dictionary type: {}", e);
                DictionaryIdent::Lindera
            }
            _ => DictionaryIdent::Lindera,
        }
    }
}
