use openssl::aes::{aes_ige, AesKey};
use openssl::symm::Mode;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;

mod iv_helpers;
mod key_management;
mod padding;

use padding::{pad, pad_remove};

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

    #[allow(unused)]
    fn new() -> AESCipher {
        let secret = iv_helpers::generate_random_buffer(32); // 128, 192, 256 bits or 16, 24, 32 bytes
        AESCipher::create(&secret)
    }

    #[allow(unused)]
    fn new_and_save_to_file<P: AsRef<std::path::Path>>(path: P) -> AESCipher {
        let secret = iv_helpers::generate_random_buffer(32); // 128, 192, 256 bits or 16, 24, 32 bytes
        let cipher = AESCipher::create(&secret);
        key_management::save_to_file(path, secret);
        cipher
    }

    pub fn new_from_shared_secret(shared_secret: &[u8; 32]) -> AESCipher {
        AESCipher::create(shared_secret)
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> AESCipher {
        let secret = key_management::load_from_file(path);
        AESCipher::create(&secret)
    }
}

trait Cipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8>;
    fn decrypt(&self, ciphertext: Vec<u8>) -> String;
}

impl Cipher for AESCipher {
    fn encrypt<T: AsRef<str>>(&self, plaintext: T) -> Vec<u8> {
        aes_encrypt(&self.encrypt_key, plaintext)
    }

    fn decrypt(&self, ciphertext: Vec<u8>) -> String {
        aes_decrypt(&self.decrypt_key, ciphertext)
    }
}

pub fn generate_keys(secret: &[u8]) -> (AesKey, AesKey) {
    let e_aeskey = AesKey::new_encrypt(secret).expect("failed to generate encrypt key");
    let d_aeskey = AesKey::new_decrypt(secret).expect("failed to generate decrypt key");
    (e_aeskey, d_aeskey)
}

pub fn aes_encrypt<T: AsRef<str>>(key: &AesKey, plaintext: T) -> Vec<u8> {
    let buffer = pad(plaintext.as_ref().as_bytes());
    let iv = iv_helpers::generate_random_iv();
    let ciphertext = aes_crypt(key, &mut iv.clone(), buffer, Mode::Encrypt);
    iv_helpers::append_iv_to_ciphertext(iv, ciphertext)
}

pub fn aes_decrypt(key: &AesKey, ciphertext: Vec<u8>) -> String {
    let (mut iv, cipher) = iv_helpers::extract_iv_and_ciphertext(ciphertext);
    let plaintext = aes_crypt(key, &mut iv, cipher, Mode::Decrypt);
    String::from_utf8_lossy(pad_remove(&plaintext)).to_string()
}

fn aes_crypt(key: &AesKey, iv: &mut [u8], input: Vec<u8>, mode: Mode) -> Vec<u8> {
    let mut output = vec![0u8; input.len()];
    aes_ige(&input, &mut output, key, iv, mode);
    output
}
