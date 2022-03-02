#[cfg(test)]
#[test]
fn test_create_new_message() {
    let message = WhiteflagMessage::new("S");
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

    assert!(message.set("EncryptionIndicator", "1"));
    assert!(!message.set("EncryptionIndicator", "2"));
    assert!(!message.set("ObjectType", "1"));

    /* Verify body fields */
    assert!(message.set("SubjectCode", "10"));
    assert!(!message.set("SubjectCode", "20"));
    assert!(message.set("ObjectType", "21"));
    assert!(!message.set("ObjectType", "22"));
    assert!(!message.set("NoField", "00"));

    /* Verify metadata */

    use crate::WhiteflagMessage;
    assertEquals(null, message.addMetadata("transactionHash", "a1b2c3"));
    assertEquals("a1b2c3", message.addMetadata("transactionHash", "d4e5f6"));
    assertEquals(null, message.addMetadata("originatorAddress", "abc123"));
    assertEquals("abc123", message.getMetadata("originatorAddress"));
}

#[test]
fn test_compile_auth_message() {}

#[test]
fn test_serialize_auth_message() {}

#[test]
fn test_deserialize_auth_message() {}

#[test]
fn test_decode_auth_message() {}
