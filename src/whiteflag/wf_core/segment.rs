use super::definitions::generic_header_fields;
use super::error::WhiteflagResult;
use super::field::Field;
use super::wf_codec::constants::BYTE;

#[derive(Clone)]
pub struct MessageSegment {
    fields: Vec<Field>,
}

impl MessageSegment {
    pub fn from(fields: Vec<Field>) -> MessageSegment {
        MessageSegment { fields }
    }

    pub fn generic_header_segment() -> MessageSegment {
        MessageSegment::from(generic_header_fields().to_vec())
    }

    /*
     * Sets all field values of this segment with values from an array
     * @since 1.1
     * @param data array with the data to be set as the field values
     * @param startIndex starting position in the array
     * @return TRUE if the data was valid and all field values are set
     * @throws WfCoreException if the provided data is invalid
     */
    pub fn set_all<T: AsRef<str> + Into<String>>(&mut self, data: &Vec<T>, start_index: usize) {
        /* int nItems = data.length - startIndex;
        if (nItems < fields.length) {
            throw new WfCoreException("Message segment has " + fields.length + " fields, but received " + nItems + " items in array", null);
        } */
        let mut index = start_index;
        for field in &mut self.fields {
            let value = &data[index];
            field.set(value.as_ref());
            index += 1;
        }

        //return this.isValid();
    }

    /**
     * Gets the value of the field specified by name
     * @param fieldname the name of the requested field
     * @return the field value, or NULL if field does not exist
     */
    pub fn get<T: AsRef<str>>(&self, field_name: T) -> Option<&String> {
        let value = self
            .fields
            .iter()
            .find(|f| f.name == field_name.as_ref())?
            .get();

        value.as_ref()
    }

    /* public final Boolean setAll(final String[] data, final int startIndex) throws WfCoreException {
        /* Check if data array contains data for all fields */
        int nItems = data.length - startIndex;
        if (nItems < fields.length) {
            throw new WfCoreException("Message segment has " + fields.length + " fields, but received " + nItems + " items in array", null);
        }
        /* Set all fields */
        int index = startIndex;
        for (WfMessageField field : fields) {
            if (Boolean.FALSE.equals(field.set(data[index]))) {
                throw new WfCoreException("Field " + field.debugInfo() + " already set or array item " + index + " contains invalid data: " + data[index], null);
            }
            index++;
        }
        return this.isValid();
    } */

    pub fn get_number_of_fields(&self) -> usize {
        self.fields.len()
    }

    /**
     * Encodes this message segment
     * @return a binary buffer with the binary encoded message segment and its bit length
     * @throws WfCoreException if the message cannot be encoded
     */
    pub fn encode(&self) -> (Vec<u8>, usize) {
        let mut buffer: Vec<u8> = vec![];
        let mut len = buffer.len();
        //let cursor = self.fields[0].start_byte;
        for field in &self.fields {
            let field_length = field.bit_length();
            buffer = super::wf_codec::common::concatinate_bits(
                buffer,
                len,
                field.encode().expect("field had no value"),
                field_length,
            );

            len += field_length;
        }

        (buffer, len)
    }
    /* @SuppressWarnings("java:S1192")
    protected final WfBinaryBuffer encode() throws WfCoreException {
        WfBinaryBuffer buffer = WfBinaryBuffer.create();
        int byteCursor = fields[0].startByte;
        for (WfMessageField field : fields) {
            if (field.startByte != byteCursor) {
                throw new WfCoreException("Invalid field order while encoding: did not expect field " + field.debugInfo() + " at byte " + byteCursor, null);
            }
            //buffer.addMessageField(field);
            buffer.appendBits(field.encode(), field.bitLength());
            byteCursor = field.endByte;
        }
        return buffer;
    } */
}
