use super::constants::{BYTE, HEXRADIX, QUADBIT};

/// Encodes a (hexa)decimal string into a binary buffer
pub fn encodeBDX(mut data: String) -> Vec<u32> {
    if data.len() % 2 == 1 {
        data = data + "0";
    }

    let input_length = data.len();
    let mut char_data = data.chars();
    let mut buffer = vec![0; input_length / 2];

    for i in (0..input_length).step_by(2) {
        let digit0: u32 = to_hex_digit(char_data.nth(i));
        let digit1: u32 = to_hex_digit(char_data.nth(i + 1));
        buffer[i / 2] = (digit0 << QUADBIT) + digit1;
    }

    buffer
}

fn to_hex_digit(data: Option<char>) -> u32 {
    data.expect("out of bounds")
        .to_digit(HEXRADIX as u32)
        .expect("failed to convert to digit")
}

fn from_hex_digit(data: u32) -> char {
    std::char::from_digit(data, HEXRADIX as u32).expect("failed to convert to char")
}

/// Decodes a binary buffer into a (hexa)decimal string
pub fn decodeBDX(buffer: Vec<u32>, bit_length: u8) -> String {
    let mut hexadecimal_string: Vec<char> = Vec::new();

    for bit_index in (0..bit_length).step_by(BYTE) {
        let byte_cursor = bit_index as usize / BYTE;
        hexadecimal_string.push(from_hex_digit((buffer[byte_cursor] >> QUADBIT) & 0xF));
        if (bit_index + (QUADBIT as u8)) < bit_length {
            hexadecimal_string.push(from_hex_digit(buffer[byte_cursor] & 0xF));
        }
    }

    hexadecimal_string.into_iter().collect()
}
