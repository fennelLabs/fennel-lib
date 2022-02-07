use openssl::aes::{aes_ige, AesKey};
use openssl::rand::rand_bytes;
use openssl::symm::Mode;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;

pub fn generate_key() -> AesKey {
    let buf = generate_buffer_16(); // 128, 192, 256 bits or 16, 24, 32 bytes
    let aeskey = AesKey::new_encrypt(&buf).expect("failed to generate key");
    aeskey
}

pub fn generate_cipher() {}

pub fn encrypt(message: String) -> Vec<u8> {
    let length = calculate_resize(&message);
    let mut m: Vec<u8> = message.into_bytes();
    m.resize(length, 0);

    let aeskey = generate_key();
    let mut iv = generate_buffer_32(); //*b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";
    let mut outbuf = vec![0; length];
    aes_ige(&m, &mut outbuf, &aeskey, &mut iv, Mode::Encrypt);
    outbuf
}

pub fn decrypt() {}

fn generate_buffer_16() -> [u8; 16] {
    let mut buf = [0; 16]; // 128, 192, 256 bits or 16, 24, 32 bytes
    rand_bytes(&mut buf).unwrap();
    buf
}

fn generate_buffer_32() -> [u8; 32] {
    let mut buf = [0; 32]; // 128, 192, 256 bits or 16, 24, 32 bytes
    rand_bytes(&mut buf).unwrap();
    buf
}

fn calculate_resize(message: &str) -> usize {
    let length = message.len();
    length + (16 - length % 16)
}