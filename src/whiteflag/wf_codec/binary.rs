use super::constants::BYTE;

pub fn encodeBIN(binary_str: String) -> Vec<u8> {
    let bit_length: usize = binary_str.len();
    let byte_length: usize = (bit_length / BYTE)
        + (match bit_length % BYTE == 0 {
            true => 0,
            false => 1,
        });
    let mut buffer = Vec::<u8>::with_capacity(byte_length);

    for bit_index in 0..bit_length {
        if binary_str.chars().nth(bit_index).expect("something wrong") == '1' {
            let byte_cursor: usize = bit_index / BYTE;
            let bit_position: usize = bit_index % BYTE;
            buffer[byte_cursor] |= 0x80 >> bit_position;
        }
    }

    buffer
}
