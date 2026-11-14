use crate::varint::{UnexpectedEndOfBufferError, VarIntDecode, VarIntEncode};

#[derive(Debug, thiserror::Error)]
pub enum ReadParseError {
    #[error("Failed to parse `read` character {0}")]
    ReadParseError(i32),
    #[error("Unexpected end of buffer while reading byte")]
    UnexpectedEndOfBuffer(#[from] UnexpectedEndOfBufferError),
}

pub(crate) fn read_to_bin(read: &str, buf: &mut Vec<u8>) {
    buf.extend((read.len() as isize).to_varint()); // isize to allow negative length for None

    buf.extend(read.chars().flat_map(|c| {
        let diff = (c as i32) - 0x30CD; // 0x30CD: 'ネ' = (0x30A1 'ァ' + 0x30FA 'ヺ') / 2
        diff.to_varint()
    }));
}

pub(crate) fn read_from_bin<I: Iterator<Item = u8>>(
    iter: &mut I,
) -> Result<Option<String>, ReadParseError> {
    let read_len: isize = iter.parse_varint()?;
    if read_len >= 0 {
        // SECURITY: Limit the maximum length of the read string to prevent potential memory exhaustion attacks.
        let mut read_str = String::with_capacity((read_len as usize).min(256)); // Arbitrary limit of 256 characters

        while read_str.len() < read_len as usize {
            let diff: i32 = iter.parse_varint()?;
            read_str.push(
                diff.checked_add(0x30CD)
                    .and_then(|c| std::char::from_u32(c as u32))
                    .ok_or(ReadParseError::ReadParseError(diff))?,
            );
        }

        Ok(Some(read_str))
    } else {
        Ok(None)
    }
}
