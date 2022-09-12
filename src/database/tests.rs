use crate::database::*;
use aes_tools::{AESCipher, FennelCipher};
use dh_tools::{get_session_public_key, get_session_secret, get_shared_secret};
use std::sync::Arc;

#[test]
fn test_insert_and_retrieve_message() {
    let db = get_message_database_handle();
    let db_2 = Arc::clone(&db);
    let message_sent = Message {
        sender_id: [2; 4],
        fingerprint: [0; 16],
        message: [0; 512],
        signature: [0; 512],
        public_key: [0; 526],
        recipient_id: [2; 4],
        message_type: [0; 1],
    };
    insert_message(db, message_sent).expect("failed message insertion");
    let result: Vec<Message> = retrieve_messages(
        db_2,
        Identity {
            id: [2; 4],
            fingerprint: [0; 16],
            public_key: [0; 526],
            shared_secret_key: [0; 32],
        },
    );
    assert_eq!(result.len(), 1);
    assert_eq!([0; 512], result[0].message);
}

#[test]
fn test_insert_and_retrieve_identity() {
    let db = get_identity_database_handle();
    let db_2 = Arc::clone(&db);
    let identity: Identity = Identity {
        id: [0; 4],
        fingerprint: [0; 16],
        public_key: [0; 526],
        shared_secret_key: [0; 32],
    };
    insert_identity(db, &identity).expect("failed identity insertion");
    let result: Identity = retrieve_identity(db_2, [0; 4]);
    assert_eq!(identity.id, result.id);
    assert_eq!(identity.fingerprint, result.fingerprint);
    assert_eq!(identity.public_key, result.public_key);
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
