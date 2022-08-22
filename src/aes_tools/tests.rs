use crate::aes_tools::*;

const MESSAGE: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
    sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi 
    ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit 
    in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur 
    sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[test]
fn test_padding_creates_multiple_16() {
    let buffer = vec![0; 13];
    let padded_buffer = padding::pad(&buffer);
    assert_eq!(16, padded_buffer.len());
}

#[test]
fn test_padding_returns_original_vector() {
    let buffer = vec![0; 13];
    let padded_buffer = padding::pad(&buffer);
    let buffer_returned = padding::pad_remove(&padded_buffer);

    assert_eq!(buffer, buffer_returned);
}

#[test]
fn test_padding_returns_original_vector_when_multiple_of_16() {
    let buffer = vec![45; 32];
    let padded_buffer = padding::pad(&buffer);
    let buffer_returned = padding::pad_remove(&padded_buffer);

    assert_eq!(buffer, buffer_returned);
}

#[test]
fn test_key_gen_without_panic() {
    AESCipher::new();
}

#[test]
fn test_encrypt_without_panic() {
    let cipher: AESCipher = AESCipher::new();
    cipher.encrypt(MESSAGE);
    ()
}

#[test]
fn test_aes_key_new() {
    let cipher: AESCipher = AESCipher::new();
    test_cipher(cipher);
}

#[test]
fn test_aes_key_load_from_file() {
    let path = "./test.txt";
    let cipher = AESCipher::new_and_save_to_file(path);
    let ciphertext = cipher.encrypt(MESSAGE);

    let cipher_from_file = AESCipher::from_file(path);
    let plaintext = cipher_from_file.decrypt(&ciphertext);

    assert_eq!(MESSAGE, plaintext)
}

fn test_cipher(cipher: AESCipher) {
    let ciphertext: Vec<u8> = cipher.encrypt(MESSAGE);
    let plaintext = cipher.decrypt(&ciphertext);
    assert_eq!(MESSAGE, plaintext);
}
