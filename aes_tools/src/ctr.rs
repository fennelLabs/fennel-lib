use crate::FennelCipher;
use openssl::symm::{decrypt, encrypt, Cipher};

pub struct AES256CTR<'a> {
    cipher: Cipher,
    key: &'a [u8],
    iv: Option<&'a [u8]>,
}

impl<'a> AES256CTR<'a> {
    pub fn new(key: &'a [u8], iv: Option<&'a [u8]>) -> AES256CTR<'a> {
        AES256CTR {
            cipher: Cipher::aes_256_ctr(),
            key,
            iv,
        }
    }
}

impl<'a> FennelCipher for AES256CTR<'a> {
    fn encrypt<T: AsRef<[u8]>>(&self, plaintext: T) -> Vec<u8> {
        encrypt(self.cipher, &self.key, self.iv, plaintext.as_ref()).unwrap()
    }

    fn decrypt<T: AsRef<[u8]>>(&self, ciphertext: T) -> Vec<u8> {
        decrypt(self.cipher, &self.key, self.iv, ciphertext.as_ref()).unwrap()
    }
}
