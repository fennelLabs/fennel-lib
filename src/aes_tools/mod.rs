use openssl::aes::{aes_ige, AesKey};
use openssl::rand::rand_bytes;
use openssl::symm::Mode;

#[cfg(test)]
mod bench;
#[cfg(test)]
mod tests;

struct AESCipher {
    key: AesKey,
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
        AESCipher {
            key: generate_key(),
            iv: generate_buffer_32()
                .try_into()
                .expect("failed to convert into Vec"),
        }
    }
}

trait Cipher {
    fn encrypt<T: AsRef<str>>(&mut self, plaintext: T) -> Vec<u8>;
    fn decrypt<T: AsRef<Vec<u8>>>(&mut self, ciphertext: T) -> Vec<u8>;
}

impl Cipher for AESCipher {
    fn encrypt<T: AsRef<str>>(&mut self, plaintext: T) -> Vec<u8> {
        encrypt(&self.key, &mut self.iv, plaintext)
    }

    fn decrypt<T: AsRef<Vec<u8>>>(&mut self, ciphertext: T) -> Vec<u8> {
        decrypt(&self.key, &mut self.iv, ciphertext)
    }
}

pub fn generate_key() -> AesKey {
    let buf = generate_buffer_16(); // 128, 192, 256 bits or 16, 24, 32 bytes
    let aeskey = AesKey::new_encrypt(&buf).expect("failed to generate key");
    aeskey
}

pub fn generate_cipher() {}

pub fn encrypt_gen_key_iv<T: AsRef<str>>(message: T) -> Vec<u8> {
    let aeskey = generate_key();
    let mut iv = generate_buffer_32(); //*b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F";

    encrypt(&aeskey, &mut iv, message)
}

pub fn encrypt<T: AsRef<str>>(key: &AesKey, iv: &mut [u8], plaintext: T) -> Vec<u8> {
    let plaintext_slice = plaintext.as_ref().as_bytes();
    let pos = plaintext_slice.len();
    let length = calculate_resize(pos);

    let mut buffer = vec![0; length];
    buffer[..pos].copy_from_slice(plaintext_slice);

    //let (length, buffer) = normalize_input(plaintext.as_ref().as_bytes().to_vec());

    let mut ciphertext = vec![0; length];
    aes_ige(&buffer, &mut ciphertext, &key, iv, Mode::Encrypt);
    ciphertext
}

pub fn decrypt<T: AsRef<Vec<u8>>>(key: &AesKey, iv: &mut [u8], ciphertext: T) -> Vec<u8> {
    /* let ciphertext_slice = ciphertext.as_ref().as_slice();
    let pos = ciphertext_slice.len();
    let length = calculate_resize(pos);

    let mut buffer = vec![0; length];
    buffer[..pos].copy_from_slice(ciphertext_slice); */

    //let (length, buffer) = normalize_input(ciphertext.as_ref().to_owned());

    let length = ciphertext.as_ref().len();

    let mut plaintext = vec![0; length];
    aes_ige(ciphertext.as_ref(), &mut plaintext, &key, iv, Mode::Decrypt);
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
