const PAD_BASE: usize = 16;
const UTF8_SPACE: u8 = 32;

/// will add padding to the end of a vector to ensure its size is a multiple of 16
pub fn pad(data: &[u8], value: Option<u8>) -> Vec<u8> {
    let length = calculate_resize(data.len());
    let mut buffer = data.to_vec();
    buffer.resize(length, value.unwrap_or(UTF8_SPACE));
    buffer
}

fn calculate_resize(size: usize) -> usize {
    size + (PAD_BASE - size % PAD_BASE)
}
