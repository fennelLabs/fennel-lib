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
        let keys = generate_keys();

        AESCipher {
            encrypt_key: keys.0,
            decrypt_key: keys.1,
            iv: generate_buffer(32),
        }
    }
}

trait Cipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8>;
    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8>;
}

impl Cipher for AESCipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8> {
        encrypt(&self.encrypt_key, self.iv.clone(), plaintext)
    }

    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        decrypt(&self.decrypt_key, self.iv.clone(), ciphertext)
    }
}

pub fn generate_keys() -> (AesKey, AesKey) {
    let buf = generate_buffer(16); // 128, 192, 256 bits or 16, 24, 32 bytes
    let e_aeskey = AesKey::new_encrypt(&buf).expect("failed to generate encrypt key");
    let d_aeskey = AesKey::new_decrypt(&buf).expect("failed to generate decrypt key");
    (e_aeskey, d_aeskey)
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

pub fn decrypt(key: &AesKey, mut iv: Vec<u8>, ciphertext: Vec<u8>) -> Vec<u8> {
    let mut plaintext = vec![0u8; ciphertext.len()];
    aes_ige(&ciphertext, &mut plaintext, &key, &mut iv, Mode::Decrypt);
    plaintext
}

fn generate_buffer(length: usize) -> Vec<u8> {
    let mut buf = vec![0; length]; // 128, 192, 256 bits or 16, 24, 32 bytes
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
