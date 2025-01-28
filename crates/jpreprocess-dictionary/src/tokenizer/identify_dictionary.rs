pub enum DictionaryIdent {
    Lindera,
    JPreprocess,
}

impl DictionaryIdent {
    pub fn from_idx_data(idx: &[u8], data: &[u8]) -> Self {
        let Some(ident) = get_dict_preamble(idx, data) else {
            return DictionaryIdent::Lindera;
        };

        if ident.starts_with("jpreprocess") {
            DictionaryIdent::JPreprocess
        } else {
            log::warn!("Unknown dictionary type: {}", ident);
            DictionaryIdent::Lindera
        }
    }
}

fn get_dict_preamble<'a>(idx: &[u8], data: &'a [u8]) -> Option<&'a str> {
    if idx.len() < 4 {
        return None;
    }
    let first_idx = u32::from_le_bytes([idx[0], idx[1], idx[2], idx[3]]) as usize;

    if data.len() < first_idx {
        return None;
    }

    match std::str::from_utf8(&data[..first_idx]) {
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
