pub(crate) fn isize_to_varint(z: isize) -> Vec<u8> {
    let mut buf = Vec::new();
    let is_negative = z < 0;
    let mut n: usize = if is_negative { -z as usize } else { z as usize };

    if is_negative {
        buf[0] |= 0x40; // Set the sign bit
    }

    buf[0] = (n & 0x3F) as u8; // Store the first 6 bits
    n >>= 6;
    if n != 0 {
        buf[0] |= 0x80; // Set the continuation bit
    }

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

pub(crate) fn varint_to_isize(buf: &[u8]) -> (isize, usize) {
    let mut n: usize = 0;
    let mut shift = 0;
    let mut i = 0;

    loop {
        let byte = buf[i];
        n |= ((byte & 0x7F) as usize) << shift;
        shift += 7;
        i += 1;
        if byte & 0x80 == 0 {
            break;
        }
    }

    let is_negative = buf[0] & 0x40 != 0;
    let z = if is_negative {
        -(n as isize)
    } else {
        n as isize
    };

    (z, i)
}
