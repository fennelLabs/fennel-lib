use super::binary::{decode_binary, encode_binary};
use super::common::shift_left;
use super::constants::*;
use super::hexadecimal::{decode_bdx, encode_bdx};
use super::latlong::encode_latlong;

//https://github.com/WhiteflagProtocol/whiteflag-java/blob/57db4b6963a4a7913afdeb596e7ce11d46d9d93b/src/main/java/org/whiteflagprotocol/java/core/WfBinaryBuffer.java#L299
pub fn to_hex(data: &Vec<u8>) -> String {
    data.iter().flat_map(|b| convert_byte_to_hex(*b)).collect()
}

fn convert_byte_to_hex(byte: u8) -> [char; 2] {
    let byte_u32 = byte as u32;
    let c1 = std::char::from_digit((byte_u32 >> QUADBIT) & 0xF, HEXRADIX as u32)
        .expect("encoding failed");
    let c2 = std::char::from_digit(byte_u32 & 0xF, HEXRADIX as u32).expect("encoding failed");
    [c1, c2]
}

/* public static final String convertToHexString(final byte[] byteArray) {
    StringBuilder hexstr = new StringBuilder();
    for (int byteIndex = 0; byteIndex < byteArray.length; byteIndex++) {
        char[] hexDigits = new char[2];
        hexDigits[0] = Character.forDigit((byteArray[byteIndex] >> QUADBIT) & 0xF, HEXRADIX);
        hexDigits[1] = Character.forDigit((byteArray[byteIndex] & 0xF), HEXRADIX);
        hexstr.append(new String(hexDigits));
    }
    return hexstr.toString().toLowerCase();
} */

pub fn encode() {}

#[derive(Clone)]
pub struct Encoding {
    pub charset: &'static str,
    pub bit_length: usize,
    pub byte_length: Option<u8>,
    pub kind: EncodingKind,
}

#[derive(Clone)]
pub enum EncodingKind {
    BIN,
    DEC,
    HEX,
    UTF8,
    DATETIME,
    DURATION,
    LAT,
    LONG,
}

impl Encoding {
    fn new(
        charset: &'static str,
        bit_length: usize,
        byte_length: Option<u8>,
        kind: EncodingKind,
    ) -> Encoding {
        Encoding {
            charset,
            bit_length,
            byte_length,
            kind,
        }
    }

    fn encode(&self, value: String) -> Vec<u8> {
        match &self.kind {
            EncodingKind::UTF8 => value.as_bytes().to_vec(),
            EncodingKind::BIN => encode_binary(value),
            EncodingKind::DEC | EncodingKind::HEX => encode_bdx(value),
            EncodingKind::DATETIME | EncodingKind::DURATION => {
                encode_bdx(value.replace("[\\-+:.A-Z]", ""))
            }
            EncodingKind::LAT | EncodingKind::LONG => encode_latlong(value),
            _ => vec![0],
        }
    }

    fn decode(&self, buffer: Vec<u8>, bit_length: usize) -> String {
        let mut s = String::new();

        match &self.kind {
            EncodingKind::UTF8 => {
                return String::from_utf8(buffer).expect("utf8 error");
            }
            EncodingKind::BIN => {
                return decode_binary(buffer, bit_length);
            }
            EncodingKind::DEC | EncodingKind::HEX => {
                return decode_bdx(buffer, bit_length);
            }
            EncodingKind::DATETIME => {
                s.push_str(decode_bdx(buffer, bit_length).as_str());

                s.insert(4, '-');
                s.insert(7, '-');
                s.insert(10, 'T');
                s.insert(13, ':');
                s.insert(16, ':');
                s.insert(19, 'Z');
            }
            EncodingKind::DURATION => {
                s.push_str(decode_bdx(buffer, bit_length).as_str());

                s.insert(0, 'P');
                s.insert(3, 'D');
                s.insert(6, 'H');
                s.insert(9, 'M');
            }
            EncodingKind::LAT | EncodingKind::LONG => {
                let sign = if ((buffer[0] >> (BYTE - 1)) & 1) == 1 {
                    '+'
                } else {
                    '-'
                };

                s.push(sign);
                s.push_str(decode_bdx(shift_left(buffer, 1), bit_length - 1).as_str());
                s.insert(s.len() - 5, '.');
            }
        }

        s
    }
}

pub const BIN: Encoding = Encoding {
    charset: "[01]",
    bit_length: BIT,
    byte_length: None,
    kind: EncodingKind::BIN,
};

pub const DEC: Encoding = Encoding {
    charset: "[0-9]",
    bit_length: QUADBIT,
    byte_length: None,
    kind: EncodingKind::DEC,
};

pub const HEX: Encoding = Encoding {
    charset: "[a-fA-F0-9]",
    bit_length: QUADBIT,
    byte_length: None,
    kind: EncodingKind::HEX,
};

pub const UTF8: Encoding = Encoding {
    charset: r"[\u0000-\u007F]",
    bit_length: OCTET,
    byte_length: None,
    kind: EncodingKind::UTF8,
};

pub const DATETIME: Encoding = Encoding {
    charset: "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z",
    bit_length: 56,
    byte_length: Some(20),
    kind: EncodingKind::DATETIME,
};

pub const DURATION: Encoding = Encoding {
    charset: "P[0-9]{2}D[0-9]{2}H[0-9]{2}M",
    bit_length: 24,
    byte_length: Some(10),
    kind: EncodingKind::DURATION,
};

pub const LAT: Encoding = Encoding {
    charset: "[+\\-][0-9]{2}\\.[0-9]{5}",
    bit_length: 29,
    byte_length: Some(9),
    kind: EncodingKind::LAT,
};

pub const LONG: Encoding = Encoding {
    charset: "[+\\-][0-9]{3}\\.[0-9]{5}",
    bit_length: 33,
    byte_length: Some(10),
    kind: EncodingKind::LONG,
};

/* protected final WfBinaryBuffer encode() throws WfCoreException {
    WfBinaryBuffer buffer = WfBinaryBuffer.create();
    int byteCursor = fields[0].startByte;
    for (WfMessageField field : fields) {
        if (field.startByte != byteCursor) {
            throw new WfCoreException("Invalid field order while encoding: did not expect field " + field.debugInfo() + " at byte " + byteCursor, null);
        }
        buffer.addMessageField(field);
        byteCursor = field.endByte;
    }
    return buffer;
} */
