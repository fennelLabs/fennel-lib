mod codec;

use codec::constants::*;

//https://github.com/WhiteflagProtocol/whiteflag-java/blob/57db4b6963a4a7913afdeb596e7ce11d46d9d93b/src/main/java/org/whiteflagprotocol/java/core/WfBinaryBuffer.java#L299
pub fn to_hex(data: Vec<u32>) -> String {
    //let hex: Vec<char> = vec![];
    data.iter().flat_map(|b| convert_byte_to_hex(*b)).collect()
    //from_digit(num: u32, radix: u32)
    //hex.into_iter().collect()
}

fn convert_byte_to_hex(byte: u32) -> [char; 2] {
    let c1 = std::char::from_digit(byte, HEXRADIX.into()).expect("encoding failed");
    let c2 = std::char::from_digit(byte, HEXRADIX.into()).expect("encoding failed");
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

pub struct Encoding {
    charset: &'static str,
    bit_length: u8,
    byte_length: Option<u8>,
}

impl Encoding {
    pub const BIN: Encoding = Encoding {
        charset: "[01]",
        bit_length: BIT,
        byte_length: None,
    };

    pub const DEC: Encoding = Encoding {
        charset: "[0-9]",
        bit_length: QUADBIT,
        byte_length: None,
    };

    pub const HEX: Encoding = Encoding {
        charset: "[a-fA-F0-9]",
        bit_length: QUADBIT,
        byte_length: None,
    };

    pub const UTF8: Encoding = Encoding {
        charset: r"[\u0000-\u007F]",
        bit_length: OCTET,
        byte_length: None,
    };

    pub const DATETIME: Encoding = Encoding {
        charset: "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z",
        bit_length: 56,
        byte_length: Some(20),
    };

    pub const DURATION: Encoding = Encoding {
        charset: "P[0-9]{2}D[0-9]{2}H[0-9]{2}M",
        bit_length: 24,
        byte_length: Some(10),
    };

    pub const LAT: Encoding = Encoding {
        charset: "[+\\-][0-9]{2}\\.[0-9]{5}",
        bit_length: 29,
        byte_length: Some(9),
    };

    pub const LONG: Encoding = Encoding {
        charset: "[+\\-][0-9]{3}\\.[0-9]{5}",
        bit_length: 33,
        byte_length: Some(10),
    };

    fn new(charset: &'static str, bit_length: u8, byte_length: Option<u8>) -> Encoding {
        Encoding {
            charset,
            bit_length,
            byte_length,
        }
    }
}

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
