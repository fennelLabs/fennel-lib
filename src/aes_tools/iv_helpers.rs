use openssl::rand::rand_bytes;

const IV_LENGTH: usize = 32;

pub fn generate_random_buffer(length: usize) -> Vec<u8> {
    let mut buf = vec![0; length];
    rand_bytes(&mut buf).unwrap();
    buf
}

pub fn generate_random_iv() -> Vec<u8> {
    generate_random_buffer(IV_LENGTH)
}

pub fn append_iv_to_ciphertext(mut iv: Vec<u8>, mut ciphertext: Vec<u8>) -> Vec<u8> {
    iv.append(&mut ciphertext);
    iv
}

pub fn extract_iv_and_ciphertext(data: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let iv = data[..IV_LENGTH].into();
    let cipher = data[IV_LENGTH..].into();

    (iv, cipher)
}
