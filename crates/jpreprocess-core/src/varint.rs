macro_rules! u_variant {
    ($t:ty,$to_varint_name:ident,$from_varint_name:ident) => {
        pub(crate) fn $to_varint_name(mut n: $t) -> Vec<u8> {
            let mut buf = Vec::new();
            buf.push((n & 0x7F) as u8);
            n >>= 7;
            while n != 0 {
                let mut byte = (n & 0x7F) as u8;
                n >>= 7;
                if n != 0 {
                    byte |= 0x80;
                }
                buf.push(byte);
            }
            buf
        }
        pub(crate) fn $from_varint_name<I: Iterator<Item = u8>>(iter: &mut I) -> $t {
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
    };
}

u_variant!(usize, usize_to_varint, varint_to_usize);

macro_rules! i_to_varint {
    ($ti:ty,$tu:ty,$to_varint_name:ident,$from_varint_name:ident) => {
        pub(crate) fn $to_varint_name(z: $ti) -> Vec<u8> {
            let mut buf = Vec::new();
            let is_negative = z < 0;
            let mut n: $tu = if is_negative { (-z) as $tu } else { z as $tu };

            let mut byte = (n & 0x3F) as u8; // Store the first 6 bits
            n >>= 6;
            if n != 0 {
                byte |= 0x80; // Set the continuation bit
            }
            if is_negative {
                byte |= 0x40; // Set the sign bit for negative numbers
            }
            buf.push(byte);

            while n != 0 {
                let mut byte = (n & 0x7F) as u8;
                n >>= 7;
                if n != 0 {
                    byte |= 0x80;
                }
                buf.push(byte);
            }

            buf
        }
        pub(crate) fn $from_varint_name<I: Iterator<Item = u8>>(iter: &mut I) -> $ti {
            let first_byte = iter
                .next()
                .expect("Unexpected end of buffer while reading signed varint");
            let mut n: $tu = (first_byte & 0x3F) as $tu;
            let mut shift = 6;

            if first_byte & 0x80 != 0 {
                loop {
                    let byte = iter
                        .next()
                        .expect("Unexpected end of buffer while reading signed varint");
                    n |= ((byte & 0x7F) as $tu) << shift;
                    shift += 7;
                    if byte & 0x80 == 0 {
                        break;
                    }
                }
            }

            let is_negative = first_byte & 0x40 != 0;
            let z = if is_negative { -(n as $ti) } else { n as $ti };

            z
        }
    };
}

i_to_varint!(isize, usize, isize_to_varint, varint_to_isize);
i_to_varint!(i32, u32, i32_to_varint, varint_to_i32);

pub(crate) fn read_u8<I: Iterator<Item = u8>>(iter: &mut I) -> u8 {
    iter.next()
        .expect("Unexpected end of buffer while reading byte")
}
