use crate::WhiteflagMessage;

#[cfg(test)]
#[test]
fn test_create_new_message() {
    let mut message = WhiteflagMessage::new("S".to_string());
    assert_eq!(message.message_type, "S");
    assert!(message.is_valid());

    assert_eq!(
        "WF",
        message.prefix
    );
    assert_eq!(
        "1",
        message.version
    );
    assert_eq!(
        "S",
        message.message_code
    );

    assert!(message.set_encryption_indicator("1".to_string()));
    assert!(!message.set_encryption_indicator("2".to_string()));
    assert!(!message.set_object_type("1".to_string()));

    /* Verify body fields */
    assert!(message.set_subject_code("10".to_string()));
    assert!(!message.set_subject_code("20".to_string()));
    assert!(message.set_object_type("21".to_string()));
    assert!(!message.set_object_type("22".to_string()));

    /* Verify metadata */
    assert_eq!(None, message.set_transaction_hash("a1b2c3".to_string()));
    assert_eq!("a1b2c3", message.set_transaction_hash("d4e5f6".to_string()).unwrap());
    assert_eq!(None, message.set_originator_address("abc123".to_string()));
    assert_eq!("abc123", message.get_originator_address());
}

#[test]
fn test_compile_auth_message() {}

#[test]
fn test_serialize_auth_message() {}

#[test]
fn test_deserialize_auth_message() {}

#[test]
fn test_decode_auth_message() {}
