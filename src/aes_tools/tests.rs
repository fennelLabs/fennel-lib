use crate::aes_tools::*;

#[test]
fn test_key_gen_without_panic() {
    generate_keys();
}

#[test]
fn test_encrypt_without_panic() {
    let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
    sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi 
    ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit 
    in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur 
    sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    let cipher: AESCipher = AESCipher::new();
    cipher.encrypt(message);
    ()
}

#[test]
fn test_message_is_same() {
    let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, 
    sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. 
    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi 
    ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit 
    in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur 
    sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    let cipher: AESCipher = AESCipher::new();

    let ciphertext: Vec<u8> = cipher.encrypt(&message);
    let plaintext = cipher.decrypt(ciphertext);

    let message_decoded = String::from_utf8(plaintext).expect("Found invalid UTF-8");

    assert_eq!(message.trim_end(), message_decoded.trim_end());
}

#[test]
fn test_padding_creates_multiple_16() {
    let buffer = vec![0; 13];
    let padded_buffer = padding::pad(&buffer, None);
    assert_eq!(16, padded_buffer.len());
}
