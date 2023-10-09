use crate::{DictionarySerializer, DictionaryStore};
use byteorder::{ByteOrder, LittleEndian};
use jpreprocess_core::{error::JPreprocessErrorKind, JPreprocessResult};

impl<'a> DictionaryStore<'a> for lindera_core::dictionary::Dictionary {
    fn get_bytes(&'a self, id: u32) -> JPreprocessResult<&'a [u8]> {
        get_bytes(id, &self.words_idx_data, &self.words_data)
    }
    fn identifier(&self) -> Option<String> {
        get_metadata(&self.words_idx_data, &self.words_data)
    }
    fn serlializer_hint(&self) -> Box<dyn DictionarySerializer> {
        detect_dictionary(&self.words_idx_data, &self.words_data)
    }
}

impl<'a> DictionaryStore<'a> for lindera_core::dictionary::UserDictionary {
    fn get_bytes(&'a self, id: u32) -> JPreprocessResult<&'a [u8]> {
        get_bytes(id, &self.words_idx_data, &self.words_data)
    }
    fn identifier(&self) -> Option<String> {
        get_metadata(&self.words_idx_data, &self.words_data)
    }
    fn serlializer_hint(&self) -> Box<dyn DictionarySerializer> {
        detect_dictionary(&self.words_idx_data, &self.words_data)
    }
}

fn get_metadata(words_idx_data: &[u8], words_data: &[u8]) -> Option<String> {
    let metadata_end = LittleEndian::read_u32(&words_idx_data[0..4]) as usize;
    if metadata_end == 0 {
        return None;
    }

    String::from_utf8(words_data[0..metadata_end].to_vec()).ok()
}

fn detect_dictionary(words_idx_data: &[u8], words_data: &[u8]) -> Box<dyn DictionarySerializer> {
    use super::serializer::{jpreprocess, lindera};

    if let Some(metadata) = get_metadata(words_idx_data, words_data) {
        let segments: Vec<&str> = metadata.split(' ').collect();
        match *segments.as_slice() {
            ["JPreprocess", "v0.3.0" | "v0.4.0" | "v0.5.0" | "v0.5.1"] => {
                return Box::new(jpreprocess::legacy_0_5_1::JPreprocessSerializer)
            }
            ["JPreprocess", ..] => return Box::new(jpreprocess::JPreprocessSerializer),
            _ => (),
        }
    }
    Box::new(lindera::LinderaSerializer)
}

fn get_bytes<'a>(
    id: u32,
    words_idx_data: &'a [u8],
    words_data: &'a [u8],
) -> JPreprocessResult<&'a [u8]> {
    let start_point = 4 * id as usize;
    if words_idx_data.len() < start_point + 4 {
        return Err(JPreprocessErrorKind::WordNotFoundError
            .with_error(anyhow::anyhow!("Word with id {:?} does not exist.", id)));
    }

    let idx = LittleEndian::read_u32(&words_idx_data[start_point..start_point + 4]) as usize;
    let idx_next = if words_idx_data.len() < start_point + 8 {
        words_data.len()
    } else {
        LittleEndian::read_u32(&words_idx_data[start_point + 4..start_point + 8]) as usize
    };

    Ok(&words_data[idx..idx_next])
}
