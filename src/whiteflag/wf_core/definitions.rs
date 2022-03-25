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
}
