use crate::database::*;
use std::sync::Arc;

#[test]
fn test_insert_and_retrieve_message() {
    let db = FennelLocalDb::new().unwrap();
    let message_sent = Message {
        sender_id: [2; 4],
        fingerprint: [0; 16],
        message: [0; 1024],
        signature: [0; 1024],
        public_key: [0; 1038],
        recipient_id: [2; 4],
    };
    db.insert_message(message_sent)
        .expect("failed message insertion");
    let result: Vec<Message> = db
        .retrieve_messages(Identity {
            id: [2; 4],
            fingerprint: [0; 16],
            public_key: [0; 1038],
            shared_secret_key: [0; 32],
        })
        .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!([0; 1024], result[0].message);
}

#[test]
fn test_insert_and_retrieve_identity() {
    let db = FennelLocalDb::new().unwrap();
    let identity: Identity = Identity {
        id: [0; 4],
        fingerprint: [0; 16],
        public_key: [0; 1038],
        shared_secret_key: [0; 32],
    };
    db.insert_identity(&identity)
        .expect("failed identity insertion");
    let result: Identity = db.retrieve_identity([0; 4]).unwrap();
    assert_eq!(identity.id, result.id);
    assert_eq!(identity.fingerprint, result.fingerprint);
    assert_eq!(identity.public_key, result.public_key);
}
