const pad_base: usize = 16;
const utf8_space: u8 = 32;

pub fn calculate_resize(size: usize) -> usize {
    size + (pad_base - size % pad_base)
}

/// ensures vector has a length that is a multiple of 16
pub fn is_size_valid(data: &[u8]) -> bool {
    data.len() % pad_base == 0
}

/// will add padding to the end of a vector to ensure its size is a multiple of 16
pub fn pad(data: &[u8], value: Option<u8>) -> Vec<u8> {
    let length = calculate_resize(data.len());
    let mut buffer = data.to_vec();
    buffer.resize(length, value.unwrap_or(utf8_space));
    buffer
}
