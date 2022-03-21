use super::constants::BYTE;

/**
 * Calculates the number of bytes required to hold the given number of bits
 */
pub fn byte_length(bit_length: isize) -> isize {
    let i_BYTE = BYTE as isize;
    (bit_length / i_BYTE) + (if (bit_length % i_BYTE) > 0 { 1 } else { 0 })
}

/**
 * Shortens the byte array to fit the length of the used bits
 */
pub fn crop_bits(buffer: Vec<u8>, bit_length: isize) -> Vec<u8> {
    if bit_length == 0 {
        return buffer;
    }

    let is_positive = bit_length > 0;
    let u_bit_length = bit_length as usize;

    let (byte_length, clear_bits) = match is_positive {
        true => {
            let length = byte_length(bit_length);
            if length > buffer.len() as isize {
                return buffer[0..length as usize].to_vec();
            }
            (length as usize, BYTE - (u_bit_length % BYTE))
        }
        false => {
            let length: isize = buffer.len() as isize + (bit_length / (BYTE as isize));
            if length < 1 {
                return vec![0];
            }
            (length as usize, u_bit_length)
        }
    };

    let mut cropped_buffer = buffer[0..byte_length].to_vec();

    /* Clear unused bits in last byte */
    if clear_bits < BYTE {
        cropped_buffer[byte_length - 1] &= 0xFF << clear_bits;
    }

    cropped_buffer
}

/**
 * Shifts bits in a byte array to the right modulo 8
 */
pub fn shift_right(buffer: Vec<u8>, shift: isize) -> Vec<u8> {
    if shift < 0 {
        return shift_left(buffer, -shift);
    }

    let modulate: usize = shift as usize % BYTE;

    if modulate == 0 {
        return buffer;
    }

    let mask: u8 = 0xFF >> (BYTE - modulate);
    let length = buffer.len() + 1;
    let mut new_buffer = vec![0; length]; //Vec::<u8>::with_capacity(length);

    for i in (1..length).rev() {
        let b = &buffer[i - 1];
        new_buffer[i] |= (0xFF & b & mask) << (BYTE - modulate);
        new_buffer[i - 1] = (0xFF & b) >> modulate;
    }

    new_buffer
}

/**
 * Shifts bits in a byte array to the left modulo 8
 */
pub fn shift_left(buffer: Vec<u8>, shift: isize) -> Vec<u8> {
    if shift < 0 {
        return shift_right(buffer, -shift);
    }

    let modulate: usize = shift as usize % BYTE;

    if modulate == 0 {
        return buffer;
    }

    let mask: u8 = 0xFF << (BYTE - modulate);
    let length = buffer.len();
    let mut new_buffer = vec![0; length];

    for i in 0..length {
        new_buffer[i] = (0xFF & buffer[i]) << modulate;
        if i < length - 1 {
            new_buffer[i] |= (0xFF & buffer[i + 1] & mask) >> (BYTE - modulate);
        }
    }

    crop_bits(new_buffer, -1 * (shift % BYTE as isize))
}
