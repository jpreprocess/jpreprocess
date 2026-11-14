#[derive(Debug, thiserror::Error)]
pub enum BinaryParseError {
    #[error("Unexpected end of buffer while reading byte")]
    UnexpectedEndOfBuffer,
    #[error("VarInt overflow: too many bytes read")]
    VarIntOverflow,
}

pub(crate) trait NextByteIterator: Iterator<Item = u8> {
    fn next_byte(&mut self) -> Result<u8, BinaryParseError>;
}

impl<I: Iterator<Item = u8>> NextByteIterator for I {
    fn next_byte(&mut self) -> Result<u8, BinaryParseError> {
        self.next().ok_or(BinaryParseError::UnexpectedEndOfBuffer)
    }
}

pub(crate) trait VarIntEncode {
    fn to_varint(self) -> impl Iterator<Item = u8>;
}

pub(crate) trait VarIntDecode<T: VarIntEncode>: Iterator<Item = u8> {
    fn parse_varint(&mut self) -> Result<T, BinaryParseError>;
}

macro_rules! u_varint {
    ($t:ty) => {
        impl VarIntEncode for $t {
            fn to_varint(self) -> impl Iterator<Item = u8> {
                struct VarIntIter($t, bool);
                impl Iterator for VarIntIter {
                    type Item = u8;
                    fn next(&mut self) -> Option<Self::Item> {
                        if self.0 == 0 && self.1 {
                            None
                        } else {
                            let mut byte = (self.0 & 0x7F) as u8;
                            self.0 >>= 7;
                            if self.0 != 0 {
                                byte |= 0x80; // Set the continuation bit
                            }
                            self.1 = true; // Mark that we've started emitting bytes
                            Some(byte)
                        }
                    }
                    fn size_hint(&self) -> (usize, Option<usize>) {
                        let leading_zeros = self.0.leading_zeros() as usize;
                        let bytes_needed = (std::mem::size_of::<$t>() * 8 - leading_zeros)
                            .div_ceil(7)
                            .max(1);
                        (bytes_needed, Some(bytes_needed))
                    }
                }
                VarIntIter(self, false)
            }
        }

        impl<T: Iterator<Item = u8>> VarIntDecode<$t> for T {
            fn parse_varint(&mut self) -> Result<$t, BinaryParseError> {
                let mut n: $t = 0;
                let mut shift = 0;

                loop {
                    let byte = self.next_byte()?;
                    n |= ((byte & 0x7F) as $t) << shift;
                    shift += 7;
                    if byte & 0x80 == 0 {
                        break;
                    }
                    if shift >= <$t>::BITS {
                        return Err(BinaryParseError::UnexpectedEndOfBuffer);
                    }
                }

                Ok(n)
            }
        }
    };
}

u_varint!(usize);
u_varint!(u32);

macro_rules! i_varint {
    ($ti:ty,$tu:ty) => {
        impl VarIntEncode for $ti {
            fn to_varint(self) -> impl Iterator<Item = u8> {
                let n = ((self << 1) ^ (self >> (std::mem::size_of::<$ti>() * 8 - 1))) as $tu; // ZigZag encoding
                <$tu>::to_varint(n)
            }
        }
        impl<T: Iterator<Item = u8>> VarIntDecode<$ti> for T {
            fn parse_varint(&mut self) -> Result<$ti, BinaryParseError> {
                let n: $tu = self.parse_varint()?;
                Ok(((n >> 1) as $ti) ^ (-((n & 1) as $ti))) // ZigZag decoding
            }
        }
    };
}

i_varint!(isize, usize);
i_varint!(i32, u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_u32() {
        let values = [0, 1, 127, 128, 255, 256, 1024, u32::MAX];
        for &value in &values {
            let varint_iter = value.to_varint();

            let expected_len = varint_iter.size_hint().0;
            let varint_bytes: Vec<u8> = varint_iter.collect();
            assert_eq!(expected_len, varint_bytes.len());

            let decoded_value: u32 = varint_bytes.into_iter().parse_varint().unwrap();
            assert_eq!(value, decoded_value, "Failed for value: {}", value);
        }
    }
    #[test]
    fn test_varint_i32() {
        let values = [0, 1, -1, 63, -64, 64, -65, 1024, -1024, i32::MAX, i32::MIN];
        for &value in &values {
            let varint_iter = value.to_varint();

            let expected_len = varint_iter.size_hint().0;
            let varint_bytes: Vec<u8> = varint_iter.collect();
            assert_eq!(expected_len, varint_bytes.len());

            let decoded_value: i32 = varint_bytes.into_iter().parse_varint().unwrap();
            assert_eq!(value, decoded_value, "Failed for value: {}", value);
        }
    }
    #[test]
    fn test_decode_overflow() {
        let mut safe_bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0x0F].into_iter();
        VarIntDecode::<u32>::parse_varint(&mut safe_bytes)
            .expect("Expected successful parse for safe bytes");
        let mut overflow_bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x0F].into_iter();
        VarIntDecode::<u32>::parse_varint(&mut overflow_bytes)
            .expect_err("Expected overflow error");
    }
}
