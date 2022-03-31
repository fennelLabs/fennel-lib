use super::constants::BYTE;

pub fn encode_binary<T: AsRef<str>>(binary_str: T) -> Vec<u8> {
    let bit_length: usize = binary_str.as_ref().len();
    let byte_length: usize = (bit_length / BYTE)
        + (match bit_length % BYTE == 0 {
            true => 0,
            false => 1,
        });
    let mut buffer = vec![0; byte_length];

    for bit_index in 0..bit_length {
        if binary_str
            .as_ref()
            .chars()
            .nth(bit_index)
            .expect("something wrong")
            == '1'
        {
            let byte_cursor: usize = bit_index / BYTE;
            let bit_position: usize = bit_index % BYTE;
            buffer[byte_cursor] |= 0x80 >> bit_position;
        }
    }

    buffer
}

pub fn decode_binary(buffer: Vec<u8>, bit_length: usize) -> String {
    let mut data: Vec<char> = Vec::new();

    for bit_index in 0..bit_length {
        let byte_cursor: usize = bit_index / BYTE;
        let bit_position: usize = bit_index % BYTE;

        if (buffer[byte_cursor] >> (BYTE - bit_position - 1) & 1) == 1 {
            data.push('1');
        } else {
            data.push('0');
        }
    }

    data.into_iter().collect() //to lower?
}
