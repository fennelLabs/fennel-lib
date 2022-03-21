use super::hexadecimal::encode_bdx;

/**
 * Encodes a datum string into binary buffer
 */
pub fn encode_latlong(datumstr: String) {
    let buffer = encode_bdx(datumstr.replace("[\\-+:.A-Z]", "").chars().collect());

    if &datumstr[0..1] == "-" {

    }

    if &datumstr[0..1] == "+" {

    }
}

/*
     * Encodes a datum string into binary buffer
     * @since 1.1
     * @param datumstr the datum string to encode
     * @return a binary buffer containing the encoded datum
     */
     /* protected static final byte[] encodeLatLong(final String datumstr) {
        /* Encode string without fixed characters */
        final String str = datumstr.replaceAll("[\\-+:.A-Z]", "");
        byte[] byteArray = encodeBDX(str); 
        
        /* Sign of lat long coordinates */
        if (datumstr.substring(0,1).equals("-")) {
            byteArray = WfBinaryBuffer.shiftRight(byteArray, 1);
        }
        if (datumstr.substring(0,1).equals("+")) {
            byteArray = WfBinaryBuffer.shiftRight(byteArray, 1);
            byteArray[0] |= (byte) 0x80;
        }
        /* Return byte array without excess bits at end */
        final int bitLength = 1 + str.length() * QUADBIT;
        return WfBinaryBuffer.cropBits(byteArray, bitLength);
    } */