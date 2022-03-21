use super::constants::QUADBIT;
use super::hexadecimal::encode_bdx;
use super::common::{shift_right, crop_bits};

/**
 * Encodes a datum string into binary buffer
 */
pub fn encode_latlong(input: String) -> Vec<u8> {
    let cleaned_input = input.replace("[\\-+:.A-Z]", "");
    let length = &cleaned_input.len();
    let mut buffer = encode_bdx(cleaned_input);

    if &input[0..1] == "-" {
        buffer = shift_right(buffer, 1);
    }

    if &input[0..1] == "+" {
        buffer = shift_right(buffer, 1);
        buffer[0] |= 0x80;
    }

    let bit_length = 1 + length * QUADBIT;
    crop_bits(buffer, bit_length as isize)
}
