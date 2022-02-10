use openssl::aes::{aes_ige, AesKey};
use openssl::rand::rand_bytes;
use openssl::symm::Mode;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;

struct AESCipher {
    encrypt_key: AesKey,
    decrypt_key: AesKey,
    iv: Vec<u8>,
}

impl AESCipher {
    /* fn new(key: AesKey, iv: Vec<u8>) -> AESCipher {
        AESCipher {
            key,
            iv
        }
    } */

    fn new() -> AESCipher {
        let keys = generate_key();

        AESCipher {
            encrypt_key: keys.0,
            decrypt_key: keys.1,
            iv: generate_buffer_32()
                .try_into()
                .expect("failed to convert into Vec"),
        }
    }
}

trait Cipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8>;
    fn decrypt<T: AsRef<Vec<u8>>>(&self, ciphertext: T) -> Vec<u8>;
}

impl Cipher for AESCipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8> {
        encrypt(&self.encrypt_key, self.iv.clone(), plaintext)
    }

    fn decrypt<T: AsRef<Vec<u8>>>(&self, ciphertext: T) -> Vec<u8> {
        decrypt(&self.decrypt_key, self.iv.clone(), ciphertext)
    }
}

pub fn generate_key() -> (AesKey, AesKey) {
    let buf = generate_buffer_16(); // 128, 192, 256 bits or 16, 24, 32 bytes
    let e_aeskey = AesKey::new_encrypt(&buf).expect("failed to generate encrypt key");
    let d_aeskey = AesKey::new_decrypt(&buf).expect("failed to generate decrypt key");
    (e_aeskey, d_aeskey)
}

pub fn generate_cipher() {}

pub fn encrypt_gen_key_iv<T: AsRef<str>>(message: T) -> Vec<u8> {
    let aeskey = generate_key();
    let mut iv = generate_buffer_32().to_vec();

    encrypt(&aeskey.0, iv, message)
}

pub fn encrypt<T: AsRef<str>>(key: &AesKey, mut iv: Vec<u8>, plaintext: T) -> Vec<u8> {
    let plaintext_slice = plaintext.as_ref().as_bytes();

    let pos = plaintext_slice.len();
    let length = calculate_resize(pos);

    let mut buffer = plaintext_slice.to_vec();
    buffer.resize(length, 32u8);

    //let (length, buffer) = normalize_input(plaintext.as_ref().as_bytes().to_vec());
    let mut ciphertext = vec![0u8; length];
    aes_ige(&buffer, &mut ciphertext, &key, &mut iv, Mode::Encrypt);
    ciphertext
}

pub fn decrypt<T: AsRef<Vec<u8>>>(key: &AesKey, mut iv: Vec<u8>, ciphertext: T) -> Vec<u8> {
    let input = ciphertext.as_ref();

    let length = input.len();
    let mut plaintext = vec![0u8; length];

    aes_ige(&input, &mut plaintext, &key, &mut iv, Mode::Decrypt);

    plaintext
}

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

fn calculate_resize(size: usize) -> usize {
    size + (16 - size % 16)
}

/// currently not working properly
fn normalize_input<T: Copy>(data: Vec<T>) -> (usize, Vec<T>) {
    let pos = data.len();
    let length = calculate_resize(pos);

    let mut buffer: Vec<T> = Vec::with_capacity(length);
    buffer[..pos].copy_from_slice(data.as_slice());
    (length, buffer)
}
