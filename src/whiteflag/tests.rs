use super::wf_core::creator::compile;
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

#[test]
fn text_decode_hex_message() {
    let message_encoded: String = "57463130a6a1f7da7067d41891592131a12a60c9053b4eb0aefe6263385da9f5b789421e1d7401009841882148a800000114c1e596006f04c050eca6420084".to_string();
    let field_values = vec![
        "WF",
        "1",
        "0",
        "1",
        "M",
        "4",
        "3efb4e0cfa83122b242634254c1920a769d615dfcc4c670bb53eb6f12843c3ae",
        "80",
        "2013-08-31T04:29:15Z",
        "P00D00H00M",
        "22",
        "+30.79658",
        "-037.82602",
        "8765",
        "3210",
        "042",
    ];
    //let message = WhiteflagMessage::compile_auth_message(field_values.clone()).unwrap();
    let basic_message = compile(field_values);
    assert_eq!(
        message_encoded,
        super::wf_codec::encoding::to_hex(&basic_message.encode()),
        "Encoding should be correct"
    );
}
