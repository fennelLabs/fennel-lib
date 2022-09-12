use std::sync::Arc;

use fennel_lib::{
    get_identity_database_handle,
    insert_identity, retrieve_identity, Identity,
};
use crate::{get_session_public_key, get_session_secret, get_shared_secret};
use aes_tools::{AESCipher, FennelCipher};

#[cfg(test)]
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

#[test]
fn try_generating_key_and_encrypting_database() {
    let secret = get_session_secret();
    let pub_key = get_session_public_key(&secret);

    let other_secret = get_session_secret();
    let other_pub_key = get_session_public_key(&other_secret);

    let shared_secret = get_shared_secret(secret, &other_pub_key);
    let other_shared_secret = get_shared_secret(other_secret, &pub_key);

    assert_eq!(shared_secret.as_bytes(), other_shared_secret.as_bytes());

    let identity_db = get_identity_database_handle();
    let identity_db_clone = Arc::clone(&identity_db);
    let identity = Identity {
        id: [0; 4],
        fingerprint: [0; 16],
        public_key: [0; 526],
        shared_secret_key: *shared_secret.as_bytes(),
    };
    insert_identity(identity_db, &identity).unwrap();
    let retrieved_identity = retrieve_identity(identity_db_clone, [0; 4]);
    let shared_secret_from_database = retrieved_identity.shared_secret_key;

    let cipher: AESCipher = AESCipher::new_from_shared_secret(&shared_secret_from_database);

    let ciphertext = cipher.encrypt("This is a test.");
    let plaintext = cipher.decrypt(ciphertext);

    assert_eq!("This is a test.", String::from_utf8_lossy(&plaintext));
}