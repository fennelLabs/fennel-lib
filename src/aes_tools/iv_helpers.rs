use openssl::rand::rand_bytes;

const IV_LENGTH: usize = 32;

/// Generates a random buffer of `length` for use in key padding.
pub fn generate_random_buffer(length: usize) -> Vec<u8> {
    let mut buf = vec![0; length];
    rand_bytes(&mut buf).unwrap();
    buf
}

/// Generates a random initialization vector for AES use.
pub fn generate_random_iv() -> Vec<u8> {
    generate_random_buffer(IV_LENGTH)
}

/// Handles sending an IV ahead of ciphertext.
pub fn append_iv_to_ciphertext(mut iv: Vec<u8>, mut ciphertext: Vec<u8>) -> Vec<u8> {
    iv.append(&mut ciphertext);
    iv
}

/// Breaks IV from ciphertext for later processing.
pub fn extract_iv_and_ciphertext(data: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let iv = data[..IV_LENGTH].into();
    let cipher = data[IV_LENGTH..].into();

    (iv, cipher)
}
