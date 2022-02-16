#[cfg(test)]

#[test]
fn try_generating_key_and_encrypting() {
    use crate::{get_ephemeral_secret, get_ephemeral_public_key, get_shared_secret, AESCipher, aes_encrypt, aes_decrypt};


    let secret = get_ephemeral_secret();
    let pub_key = get_ephemeral_public_key(&secret);

    let other_secret = get_ephemeral_secret();
    let other_pub_key = get_ephemeral_public_key(&other_secret);

    let shared_secret = get_shared_secret(secret, &other_pub_key);
    let other_shared_secret = get_shared_secret(other_secret, &pub_key);

    assert_eq!(shared_secret.as_bytes(), other_shared_secret.as_bytes());

    let cipher: AESCipher = AESCipher::new_from_shared_secret(shared_secret);

    let ciphertext = aes_encrypt(&cipher.encrypt_key, cipher.iv.clone(), String::from("This is a test."));
    let plaintext = aes_decrypt(&cipher.decrypt_key, cipher.iv, ciphertext);

    assert_eq!(String::from("This is a test."), plaintext);
}