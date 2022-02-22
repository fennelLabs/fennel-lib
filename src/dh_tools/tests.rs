use crate::{FennelLocalDb, Identity};

#[cfg(test)]
#[test]
fn try_generating_key_and_encrypting() {
    use crate::{
        aes_decrypt, aes_encrypt, get_session_public_key, get_session_secret, get_shared_secret,
        AESCipher,
    };

    let secret = get_session_secret();
    let pub_key = get_session_public_key(&secret);

    let other_secret = get_session_secret();
    let other_pub_key = get_session_public_key(&other_secret);

    let shared_secret = get_shared_secret(secret, &other_pub_key);
    let other_shared_secret = get_shared_secret(other_secret, &pub_key);

    assert_eq!(shared_secret.as_bytes(), other_shared_secret.as_bytes());

    let cipher: AESCipher = AESCipher::new_from_shared_secret(shared_secret.as_bytes());

    let ciphertext = aes_encrypt(
        &cipher.encrypt_key,
        cipher.iv.clone(),
        String::from("This is a test."),
    );
    let plaintext = aes_decrypt(&cipher.decrypt_key, cipher.iv, ciphertext);

    assert_eq!(String::from("This is a test."), plaintext);
}

#[test]
fn try_generating_key_and_encrypting_database() {
    use crate::{
        aes_decrypt, aes_encrypt, get_session_public_key, get_session_secret, get_shared_secret,
        AESCipher,
    };

    let secret = get_session_secret();
    let pub_key = get_session_public_key(&secret);

    let other_secret = get_session_secret();
    let other_pub_key = get_session_public_key(&other_secret);

    let shared_secret = get_shared_secret(secret, &other_pub_key);
    let other_shared_secret = get_shared_secret(other_secret, &pub_key);

    assert_eq!(shared_secret.as_bytes(), other_shared_secret.as_bytes());

    let identity_db = FennelLocalDb::new().unwrap();
    let identity = Identity {
        id: [0; 4],
        fingerprint: [0; 16],
        public_key: [0; 1038],
        shared_secret_key: *shared_secret.as_bytes(),
    };
    identity_db.insert_identity(&identity).unwrap();
    let retrieved_identity = identity_db.retrieve_identity([0; 4]).unwrap();
    let shared_secret_from_database = retrieved_identity.shared_secret_key;

    let cipher: AESCipher = AESCipher::new_from_shared_secret(&shared_secret_from_database);

    let ciphertext = aes_encrypt(
        &cipher.encrypt_key,
        cipher.iv.clone(),
        String::from("This is a test."),
    );
    let plaintext = aes_decrypt(&cipher.decrypt_key, cipher.iv, ciphertext);

    assert_eq!(String::from("This is a test."), plaintext);
}
