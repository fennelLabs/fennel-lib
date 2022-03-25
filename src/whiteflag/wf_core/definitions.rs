use crate::whiteflag::wf_codec::encoding::*;
use crate::whiteflag::wf_core::field::Field;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref generic_header_fields: [Field; 7] = [
        Field::new("Prefix", Regex::new("^WF$").ok(), UTF8, 0, 2),
        Field::new("Version", Regex::new("(?=1)^[A-Z0-9]{1}$").ok(), UTF8, 2, 3),
        Field::new(
            "EncryptionIndicator",
            Regex::new("(?=0|1|2)^[A-Z0-9]{1}$").ok(),
            UTF8,
            3,
            4,
        ),
        Field::new("DuressIndicator", Regex::new("^[0-1]{1}$").ok(), BIN, 4, 5),
        Field::new(
            "MessageCode",
            Regex::new("(?=A|K|T|P|E|S|D|I|M|Q|R|F)^[A-Z]{1}$").ok(),
            UTF8,
            5,
            6,
        ),
        Field::new(
            "ReferenceIndicator",
            Regex::new(
                ["(?=0|1|2|3|4|5|6|7|8|9)^", HEX.charset, "{1}$"]
                    .concat()
                    .as_str(),
            )
            .ok(),
            HEX,
            6,
            7,
        ),
        Field::new(
            "ReferencedMessage",
            Regex::new(["^", HEX.charset, "{64}$"].concat().as_str()).ok(),
            HEX,
            7,
            71,
        ),
    ];

    static ref authentication_body_fields: [Field; 2] = [
        Field::new(
            "VerificationMethod",
            Regex::new(["(?=1|2)^", HEX.charset, "{1}$"].concat().as_str()).ok(),
            HEX,
            71,
            72
        ),
        Field::new(
            "VerificationData",
            Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(),
            UTF8,
            72,
            -1
        )
    ];

    static ref crypto_body_fields: [Field; 2] = [
        Field::new("CryptoDataType", Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(), HEX, 71, 73),
        Field::new("CryptoData", Regex::new(["^", HEX.charset, "*$"].concat().as_str()).ok(), HEX, 73, -1),
    ];

    static ref freetext_body_fields: [Field; 1] = [
        Field::new("Text", Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok(), UTF8, 71, -1),
    ];

    static ref resource_body_fields: [Field; 2] = [
        Field::new("ResourceMethod", Regex::new(["(?=1)^", HEX.charset, "{1}$"].concat().as_str()).ok() , HEX, 71, 72),
        Field::new("ResourceData", Regex::new(["^", UTF8.charset, "*$"].concat().as_str()).ok() , UTF8, 72, -1),
    ];

    static ref test_body_fields: [Field; 1] = [
        Field::new("PseudoMessageCode", Regex::new("^[A-Z]{1}$").ok(), UTF8, 71, 72),
    ];

    static ref sign_signal_body_fields: [Field; 9] = [
        Field::new("SubjectCode", Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok() , HEX, 71, 73),
        Field::new("DateTime", Regex::new(["^", DATETIME.charset, "$"].concat().as_str()).ok(), DATETIME, 73, 93),
        Field::new("Duration", Regex::new(["^", DURATION.charset, "$"].concat().as_str()).ok(), DURATION, 93, 103),
        Field::new("ObjectType", Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(), HEX, 103, 105),
        Field::new("ObjectLatitude", Regex::new(["^", LAT.charset, "$"].concat().as_str()).ok(), LAT, 105, 114),
        Field::new("ObjectLongitude", Regex::new(["^", LONG.charset, "$"].concat().as_str()).ok(), LONG, 114, 124),
        Field::new("ObjectSizeDim1", Regex::new(["^", DEC.charset, "{4}$"].concat().as_str()).ok(), DEC, 124, 128),
        Field::new("ObjectSizeDim2", Regex::new(["^", DEC.charset, "{4}$"].concat().as_str()).ok(), DEC, 128, 132),
        Field::new("ObjectOrientation", Regex::new(["^", DEC.charset, "{3}$"].concat().as_str()).ok(), DEC, 132, 135)
    ];

    static ref request_fields: [Field; 2] = [
        Field::new("ObjectType", Regex::new(["^", HEX.charset, "{2}$"].concat().as_str()).ok(), HEX, 135, 137),
        Field::new("ObjectTypeQuant", Regex::new(["^", DEC.charset, "{2}$"].concat().as_str()).ok() , DEC, 137, 139)
    ];
}
