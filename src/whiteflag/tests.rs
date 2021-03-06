use crate::whiteflag::wf_core::message::WhiteflagMessage;

#[cfg(test)]
#[test]
fn test_create_new_message() {
    let mut message = WhiteflagMessage::new("S".to_string());
    assert_eq!(message.message_type, "S");
    assert!(message.is_valid());

    assert_eq!("WF", message.prefix);
    assert_eq!("1", message.version);
    assert_eq!("S", message.message_code);

    assert!(message.set_encryption_indicator("1".to_string()));
    assert!(!message.set_encryption_indicator("2".to_string()));

    /* Verify body fields */
    assert!(message.set_subject_code("10".to_string()));
    assert!(!message.set_subject_code("20".to_string()));
    assert!(message.set_object_type("21".to_string()));
    assert!(!message.set_object_type("22".to_string()));

    /* Verify metadata */
    assert_eq!(None, message.set_transaction_hash("a1b2c3".to_string()));
    assert_eq!(
        "a1b2c3",
        message.set_transaction_hash("d4e5f6".to_string()).unwrap()
    );
    assert_eq!(None, message.set_originator_address("abc123".to_string()));
    assert_eq!("abc123", message.get_originator_address());
}

#[test]
fn test_compile_auth_message() {
    let field_values = vec![
        "WF",
        "1",
        "0",
        "0",
        "A",
        "0",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "1",
        "b01218a30dd3c23d050af254bfcce31a715fecdff6a23fd59609612e6e0ef263",
    ];

    let message = WhiteflagMessage::compile_auth_message(field_values.clone()).unwrap();

    assert_eq!("A", message.message_type());
    assert_eq!(field_values[0], message.prefix());
    assert_eq!(field_values[1], message.version());
    assert_eq!(field_values[2], message.get_encryption_indicator());
    assert_eq!(field_values[3], message.duress_indictor());
    assert_eq!(field_values[4], message.message_code());
    assert_eq!(field_values[5], message.reference_indicator());
    assert_eq!(field_values[6], message.referenced_message());
    assert_eq!(field_values[7], message.verification_method());
    assert_eq!(field_values[8], message.verification_data());
    assert!(message.is_valid());
}

#[test]
fn test_serialize_auth_message() {}

#[test]
fn test_deserialize_auth_message() {}

#[test]
fn test_decode_auth_message() {}
