pub(crate) trait VarInt {
    fn to_varint(self) -> impl Iterator<Item = u8>;
    fn from_varint<I: Iterator<Item = u8>>(iter: &mut I) -> Self;
}

macro_rules! u_varint {
    ($t:ty) => {
        impl VarInt for $t {
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
                            .min(1);
                        (bytes_needed, Some(bytes_needed))
                    }
                }
                VarIntIter(self, false)
            }

            fn from_varint<I: Iterator<Item = u8>>(iter: &mut I) -> Self {
                let mut n: $t = 0;
                let mut shift = 0;

                loop {
                    let byte = iter
                        .next()
                        .expect("Unexpected end of buffer while reading varint");
                    n |= ((byte & 0x7F) as $t) << shift;
                    shift += 7;
                    if byte & 0x80 == 0 {
                        break;
                    }
                }

                n
            }
        }
    };
}

u_varint!(usize);
u_varint!(u32);

macro_rules! i_varint {
    ($ti:ty,$tu:ty) => {
        impl VarInt for $ti {
            fn to_varint(self) -> impl Iterator<Item = u8> {
                let n = ((self << 1) ^ (self >> (std::mem::size_of::<$ti>() * 8 - 1))) as $tu; // ZigZag encoding
                <$tu>::to_varint(n)
            }

            fn from_varint<I: Iterator<Item = u8>>(iter: &mut I) -> Self {
                let n = <$tu>::from_varint(iter);
                ((n >> 1) as $ti) ^ (-((n & 1) as $ti)) // ZigZag decoding
            }
        }
    };
}

i_varint!(isize, usize);
i_varint!(i32, u32);

pub(crate) fn read_u8<I: Iterator<Item = u8>>(iter: &mut I) -> u8 {
    iter.next()
        .expect("Unexpected end of buffer while reading byte")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varint_u32() {
        let values = [0, 1, 127, 128, 255, 256, 1024, u32::MAX];
        for &value in &values {
            let varint_bytes: Vec<u8> = value.to_varint().collect();
            let decoded_value = u32::from_varint(&mut varint_bytes.into_iter());
            assert_eq!(value, decoded_value, "Failed for value: {}", value);
        }
    }
    #[test]
    fn test_varint_i32() {
        let values = [
            0,
            1,
            -1,
            127,
            -128,
            128,
            -129,
            1024,
            -1024,
            i32::MAX,
            i32::MIN,
        ];
        for &value in &values {
            let varint_bytes: Vec<u8> = value.to_varint().collect();
            let decoded_value = i32::from_varint(&mut varint_bytes.into_iter());
            assert_eq!(value, decoded_value, "Failed for value: {}", value);
        }
    }
}
