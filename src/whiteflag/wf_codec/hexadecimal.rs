use super::constants::{HEXRADIX, QUADBIT};

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
        .clone()
        .to_digit(HEXRADIX as u32)
        .expect("failed to convert to digit")
}

/* /**
 * Encodes a (hexa)decimal string into a binary buffer
 * @since 1.1
 * @param bdxstr the (hexa)decimal string to encode
 * @return a binary buffer containing the encoded (hexa)decimal string
 */
protected static final byte[] encodeBDX(final String bdxstr) {
    /* Prepare string by removing prefix and adding trailing 0 */
    String str = bdxstr;
    if (bdxstr.length() % 2 == 1) str = str + "0";

    /* Loop through hexadecimal string and take two chars at a time*/
    final int strLength = str.length();
    byte[] byteArray = new byte[strLength / 2];
    for (int i = 0; i < strLength; i += 2) {
        byteArray[i / 2] = (byte) ((Character.digit(str.charAt(i), HEXRADIX) << QUADBIT)
                                    + Character.digit(str.charAt(i + 1), HEXRADIX));
    }
    return byteArray;
}

/**
 * Decodes a binary buffer into a (hexa)decimal string
 * @since 1.1
 * @param buffer the binary buffer containing the binary encoded (hexa)decimals to decode
 * @param bitLength the buffer length, i.e. the number of bits in the buffer to decode
 * @return a (hexa)decimal string with the decoded data
 */
protected static final String decodeBDX(final byte[] buffer, final int bitLength) {
    StringBuilder str = new StringBuilder();

    /* Loop through the bits in the binary buffer */
    for (int bitIndex = 0; bitIndex < bitLength; bitIndex += BYTE) {
        final int byteCursor = bitIndex / BYTE;

        /* Add first half of byte to string */
        str.append(Character.forDigit((buffer[byteCursor] >> QUADBIT) & 0xF, HEXRADIX));
        if ((bitIndex + QUADBIT) < bitLength) {
            /* Add second half of byte to string */
            str.append(Character.forDigit((buffer[byteCursor] & 0xF), HEXRADIX));
        }
    }
    return str.toString().toLowerCase();
} */
