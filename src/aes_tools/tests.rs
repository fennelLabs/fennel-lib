use crate::aes_tools::*;

#[test]
fn test_key_gen_without_panic() {
    generate_keys();
}

#[test]
fn test_encrypt_without_panic() {
    let message = "
    {
        \"id\": 1,
        \"name\": \"xyzab\"
    }";

    let cipher: AESCipher = AESCipher::new();
    cipher.encrypt(message);
    ()
}

#[test]
fn test_message_is_same() {
    let message = "
    {
        \"id\": 1,
        \"name\": \"xyzab\"
    }";

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
