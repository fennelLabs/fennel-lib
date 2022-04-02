use super::constants::*;

//https://github.com/WhiteflagProtocol/whiteflag-java/blob/57db4b6963a4a7913afdeb596e7ce11d46d9d93b/src/main/java/org/whiteflagprotocol/java/core/WfBinaryBuffer.java#L299
pub fn to_hex(data: &Vec<u8>) -> String {
    data.iter().flat_map(|b| convert_byte_to_hex(*b)).collect()
}

fn convert_byte_to_hex(byte: u8) -> [char; 2] {
    let byte_u32 = byte as u32;
    let c1 = std::char::from_digit((byte_u32 >> QUADBIT) & 0xF, HEXRADIX as u32)
        .expect("encoding failed");
    let c2 = std::char::from_digit(byte_u32 & 0xF, HEXRADIX as u32).expect("encoding failed");
    [c1, c2]
}

/**
 * decodes a hexadecimal string into a buffer and includes bit_length
 * the equivalent to WfBinaryBuffer.convertToByteArray in whiteflag java
 */
pub fn decode_from_hexadecimal<T: AsRef<str>>(data: T) -> (Vec<u8>, usize) {
    let buffer = hex::decode(remove_hexadecimal_prefix(data.as_ref())).unwrap();
    let bit_length = buffer.len() & BYTE;
    (buffer, bit_length)
}

/**
 * removes characters from string that are invalid in hexadecimal format
 */
pub fn remove_all_invalid_hex_characters<T: AsRef<str>>(data: T) -> String {
    let re = regex::Regex::new("[-+:.A-Z]").unwrap();
    re.replace_all(data.as_ref(), "").to_string()
}

pub fn remove_hexadecimal_prefix(data: &str) -> &str {
    if data.starts_with("0x") {
        return &data[2..];
    }

    data
}

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
pub fn shift_right(buffer: &[u8], shift: isize) -> Vec<u8> {
    if shift < 0 {
        return shift_left(buffer, -shift);
    }

    let modulate: usize = shift as usize % BYTE;

    if modulate == 0 {
        return buffer.to_vec();
    }

    let mask: u8 = 0xFF >> (BYTE - modulate);
    let length = buffer.len() + 1;
    let mut new_buffer = vec![0; length];

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
pub fn shift_left(buffer: &[u8], shift: isize) -> Vec<u8> {
    if shift < 0 {
        return shift_right(buffer, -shift);
    }

    let modulate: usize = shift as usize % BYTE;

    if modulate == 0 {
        return buffer.to_vec();
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

    crop_bits(new_buffer, -(shift % BYTE as isize))
}

/**
 * Returns a byte array with a subset of the bits in the buffer
 * @param startBit the first bit of the subset to extract
 * @param bitLength the length of the subset, i.e. the number of bits to extract
 * @return a byte array with the extracted bits
 */
pub fn extract_bits(
    buffer: &[u8],
    buffer_bit_length: usize,
    start_bit: usize,
    mut bit_length: usize,
) -> Vec<u8> {
    if bit_length > (buffer_bit_length - start_bit) {
        bit_length = buffer_bit_length - start_bit;
    }

    let start_byte = start_bit / BYTE;
    let byte_length = byte_length(bit_length as isize) as usize;
    let shift = start_bit % BYTE;
    let mask = 0xFF << (BYTE - shift);

    let mut new_byte_array: Vec<u8> = vec![0; byte_length];
    if shift == 0 {
        /* Faster loop if no shift needed */
        for byte_index in 0..byte_length {
            new_byte_array[byte_index] = buffer[start_byte + byte_index];
        }
    } else {
        /* Loop through bytes to shift */
        for byte_index in 0..byte_length {
            new_byte_array[byte_index] = (0xFF & buffer[start_byte + byte_index]) << shift;
        }

        let end_byte = if byte_length < (buffer.len() - start_byte) {
            byte_length
        } else {
            byte_length - 1
        };

        for byte_index in 0..end_byte {
            new_byte_array[byte_index] |=
                (0xFF & buffer[start_byte + byte_index + 1] & mask) >> (BYTE - shift);
        }
    }

    crop_bits(new_byte_array, bit_length as isize)
}

/**
 * Appends the specified number of bits from a bytes array to the binary buffer
 * @param byteArray the byte array with the bits to be appended
 * @param nBits the number of bits to be appended from the byte array
 * @return this binary buffer
 * @throws IllegalStateException if the buffer is marked complete and cannot be altered
 */
pub fn append_bits(
    buffer_1: &[u8],
    len_1: usize,
    buffer_2: &[u8],
    mut len_2: usize,
) -> (Vec<u8>, usize) {
    /* Check number of bits */
    let max_number_of_bits = buffer_2.len() * BYTE;
    if len_2 > max_number_of_bits {
        len_2 = max_number_of_bits;
    }

    /* Add bits to the end of the buffer */
    let new_buffer = concatinate_bits(&buffer_1, len_1, &buffer_2, len_2);

    (new_buffer, len_1 + len_2)
}

/**
 * Concatinates two bitsets
 * @param byte_array_1 byte array containing the first bitset
 * @param n_bits_1 number of bits in the first bitset, i.e. which bits to take from the first byte array
 * @param byte_array_2 byte array containing the second bitset
 * @param n_bits_2 number of bits in the second bitset, i.e. which bits to take from the second byte array
 * @return a new byte array with the concatinated bits
 */
pub fn concatinate_bits(
    byte_array_1: &[u8],
    mut n_bits_1: usize,
    byte_array_2: &[u8],
    mut n_bits_2: usize,
) -> Vec<u8> {
    /* Check number of bits */
    if n_bits_1 > (byte_array_1.len() * BYTE) {
        n_bits_1 = byte_array_1.len() * BYTE;
    }

    if n_bits_2 > (byte_array_2.len() * BYTE) {
        n_bits_2 = byte_array_2.len() * BYTE;
    }

    /* Calculate parameters */
    let shift = n_bits_1 % BYTE;
    let free_bits = if shift == 0 { 0 } else { BYTE - shift };
    let byte_length_1 = (n_bits_1 / BYTE) + (if free_bits == 0 { 0 } else { 1 });
    let bit_length = n_bits_1 + n_bits_2;
    let byte_length = byte_length(bit_length as isize) as usize;

    /* Prepare byte arrays */
    let byte_array_2_shift = shift_right(&byte_array_2, shift as isize);
    let mut new_byte_array = vec![0; byte_length as usize];

    /* Concatination */
    let mut byte_cursor = 0;
    let mut start_byte_2 = 0;
    if byte_length_1 != 0 {
        /* Add first byte array */
        for byte_index in 0..byte_length_1 {
            byte_cursor = byte_index;
            new_byte_array[byte_cursor] = byte_array_1[byte_index];
        }

        /* Add overlapping byte from second byte array*/
        if free_bits > 0 {
            new_byte_array[byte_cursor] |= byte_array_2_shift[0];
            start_byte_2 = 1;
        }
        byte_cursor += 1;
    }
    /* Add the rest of the second byte array */
    let end_byte_2 = start_byte_2 + byte_length - byte_cursor;

    for byte_index in start_byte_2..end_byte_2 {
        new_byte_array[byte_cursor] = byte_array_2_shift[byte_index];
        byte_cursor += 1;
    }

    return crop_bits(new_byte_array, bit_length as isize);
}
