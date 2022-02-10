const PAD_BASE: usize = 16;

/// will add padding to the end of a vector to ensure its size is a multiple of 16
/// the number that gets added indicates how much padding was added
pub fn pad(data: &[u8]) -> Vec<u8> {
    let current_length = data.len();
    let next_multiple_length = calculate_resize(data.len());
    let mut buffer = data.to_vec();
    buffer.resize(
        next_multiple_length,
        (next_multiple_length - current_length) as u8,
    );
    buffer
}

fn calculate_resize(size: usize) -> usize {
    size + (PAD_BASE - size % PAD_BASE)
}

/// removes padding based on padding number
pub fn pad_remove(data: &[u8]) -> &[u8] {
    let length = data.len();
    let number_of_bytes_added: usize = data[length - 1].into();
    &data[0..(length - number_of_bytes_added)]
}
