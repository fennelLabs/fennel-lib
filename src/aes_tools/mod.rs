use openssl::aes::{aes_ige, AesKey};
use openssl::symm::Mode;

#[cfg(test)]
mod tests;

mod iv_helpers;
mod key_management;
mod padding;

use padding::{pad, pad_remove};

/// Represents encryption and decryption resources.
pub struct AESCipher {
    pub encrypt_key: AesKey,
    pub decrypt_key: AesKey,
}

impl AESCipher {
    fn create(secret: &[u8]) -> AESCipher {
        let keys = generate_keys(secret);

        AESCipher {
            encrypt_key: keys.0,
            decrypt_key: keys.1,
        }
    }

    /// Generates a random AES cipher.
    #[allow(unused)]
    fn new() -> AESCipher {
        let secret = iv_helpers::generate_random_buffer(32); // 128, 192, 256 bits or 16, 24, 32 bytes
        AESCipher::create(&secret)
    }

    /// Generates a random AES cipher and commits to disk.
    #[allow(unused)]
    fn new_and_save_to_file<P: AsRef<std::path::Path>>(path: P) -> AESCipher {
        let secret = iv_helpers::generate_random_buffer(32); // 128, 192, 256 bits or 16, 24, 32 bytes
        let cipher = AESCipher::create(&secret);
        key_management::save_to_file(path, secret);
        cipher
    }

    /// Generates an AES cipher from a known shared secret.
    pub fn new_from_shared_secret(shared_secret: &[u8; 32]) -> AESCipher {
        AESCipher::create(shared_secret)
    }

    /// Reads an AES cipher in from disk.
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> AESCipher {
        let secret = key_management::load_from_file(path);
        AESCipher::create(&secret)
    }
}

trait Cipher {
    fn encrypt<T: AsRef<[u8]>>(&self, plaintext: T) -> Vec<u8>;
    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8>;
}

impl Cipher for AESCipher {
    fn encrypt<T: AsRef<[u8]>>(&self, plaintext: T) -> Vec<u8> {
        aes_encrypt(&self.encrypt_key, plaintext)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        aes_decrypt(&self.decrypt_key, ciphertext)
    }
}

pub fn generate_keys(secret: &[u8]) -> (AesKey, AesKey) {
    let e_aeskey = AesKey::new_encrypt(secret).expect("failed to generate encrypt key");
    let d_aeskey = AesKey::new_decrypt(secret).expect("failed to generate decrypt key");
    (e_aeskey, d_aeskey)
}

pub fn aes_encrypt<T: AsRef<[u8]>>(key: &AesKey, plaintext: T) -> Vec<u8> {
    let buffer = pad(plaintext.as_ref());
    let iv = iv_helpers::generate_random_iv();
    let ciphertext = aes_crypt(key, iv.clone().as_mut(), &buffer, Mode::Encrypt);
    iv_helpers::append_iv_to_ciphertext(iv, ciphertext)
}

pub fn aes_decrypt(key: &AesKey, ciphertext: &[u8]) -> Vec<u8> {
    let (iv, cipher) = iv_helpers::extract_iv_and_ciphertext(ciphertext);
    let mut plaintext = aes_crypt(key, iv.to_owned().as_mut(), cipher, Mode::Decrypt);
    pad_remove(plaintext.as_mut());
    plaintext
}

fn aes_crypt(key: &AesKey, iv: &mut [u8], input: &[u8], mode: Mode) -> Vec<u8> {
    let mut output = vec![0u8; input.len()];
    aes_ige(&input, &mut output, key, iv, mode);
    output
}
