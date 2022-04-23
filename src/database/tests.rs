use crate::database::*;
use std::sync::Arc;

#[test]
fn test_insert_and_retrieve_message() {
    let db = get_message_database_handle();
    let db_2 = Arc::clone(&db);
    let message_sent = Message {
        sender_id: [2; 4],
        fingerprint: [0; 16],
        message: [0; 1024],
        signature: [0; 1024],
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
    assert_eq!([0; 1024], result[0].message);
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
