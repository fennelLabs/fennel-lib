use crate::{get_session_public_key, get_session_secret, get_shared_secret};
use aes_tools::{AESCipher, FennelCipher};

#[test]
fn try_generating_key_and_encrypting() {
    let secret = get_session_secret();
    let pub_key = get_session_public_key(&secret);

    let other_secret = get_session_secret();
    let other_pub_key = get_session_public_key(&other_secret);

    let shared_secret = get_shared_secret(secret, &other_pub_key);
    let other_shared_secret = get_shared_secret(other_secret, &pub_key);

    assert_eq!(shared_secret.as_bytes(), other_shared_secret.as_bytes());

    let cipher: AESCipher = AESCipher::new_from_shared_secret(shared_secret.as_bytes());

    let ciphertext = cipher.encrypt("This is a test.");
    let plaintext = cipher.decrypt(ciphertext);

    assert_eq!("This is a test.", String::from_utf8_lossy(&plaintext));
}
