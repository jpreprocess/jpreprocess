pub(crate) enum DictionaryIdent {
    Lindera,
    JPreprocess,
}

impl DictionaryIdent {
    pub fn from_idx_data(idx: &[u8], data: &[u8]) -> Self {
        if idx.len() < 4 {
            return DictionaryIdent::Lindera;
        }
        let first_idx = u32::from_le_bytes([idx[0], idx[1], idx[2], idx[3]]) as usize;

        if data.len() < first_idx {
            return DictionaryIdent::Lindera;
        }

        match std::str::from_utf8(&data[first_idx..]) {
            Ok(s) if s.starts_with("JPreprocess") => {
                log::info!("JPreprocess dictionary {} detected", s);
                return DictionaryIdent::JPreprocess;
            }
            Ok(s) => {
                log::warn!("Unknown dictionary type: {}", s);
            }
            Err(e) => {
                log::warn!("Error parsing dictionary type: {}", e);
            }
        }

        DictionaryIdent::Lindera
    }
}
