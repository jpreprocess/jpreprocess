pub fn get_word_data<'a>(idx: &[u8], data: &'a [u8], word_id: Option<usize>) -> Option<&'a [u8]> {
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
