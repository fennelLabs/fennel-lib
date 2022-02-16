const PAD_BASE: usize = 16;

/// will add padding to the end of a vector to ensure its size is a multiple of 16
/// the number that gets added indicates how much padding was added
pub fn pad(data: &[u8]) -> Vec<u8> {
    if is_valid_size(data) {
        return data.into();
    }

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

fn is_valid_size(data: &[u8]) -> bool {
    data.len() % PAD_BASE == 0
}

/// removes padding based on padding number
pub fn pad_remove(data: &[u8]) -> &[u8] {
    let length = data.len();
    let number_of_bytes_added: usize = data[length - 1].into();

    if number_of_bytes_added > (PAD_BASE - 1) || number_of_bytes_added > length {
        return data;
    }

    let pos_of_original_vector = match length.checked_sub(number_of_bytes_added) {
        Some(x) => x,
        None => return data,
    };

    if pos_of_original_vector > 0 && pos_of_original_vector < length {
        &data[0..(pos_of_original_vector)]
    } else {
        data
    }
}
