pub trait PrefixDictionary {
    fn dict_ident(&self) -> Option<&str>;
    fn word_data(&self, word_id: usize) -> Option<&[u8]>;
}

impl PrefixDictionary for &lindera_dictionary::dictionary::prefix_dictionary::PrefixDictionary {
    fn dict_ident(&self) -> Option<&str> {
        get_word_data(&self.words_idx_data, &self.words_data, None)
            .and_then(|ident| std::str::from_utf8(ident).ok())
    }
    fn word_data(&self, word_id: usize) -> Option<&[u8]> {
        get_word_data(&self.words_idx_data, &self.words_data, Some(word_id))
    }
}

impl PrefixDictionary for &lindera_dictionary::dictionary::prefix_dictionary::UserPrefixDictionary {
    fn dict_ident(&self) -> Option<&str> {
        get_word_data(&self.words_idx_data, &self.words_data, None)
            .and_then(|ident| std::str::from_utf8(ident).ok())
    }
    fn word_data(&self, word_id: usize) -> Option<&[u8]> {
        get_word_data(&self.words_idx_data, &self.words_data, Some(word_id))
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

    if range.end <= data.len() {
        Some(&data[range])
    } else {
        None
    }
}
