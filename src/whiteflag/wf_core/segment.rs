use super::definitions::generic_header_fields;
use super::error::WhiteflagResult;
use super::field::Field;

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
    pub fn set_all(&mut self, data: Vec<String>, start_index: usize) {
        /* int nItems = data.length - startIndex;
        if (nItems < fields.length) {
            throw new WfCoreException("Message segment has " + fields.length + " fields, but received " + nItems + " items in array", null);
        } */
        let mut index = start_index;
        for field in &mut self.fields {
            let value = &data[index];
            field.set(value.to_owned());
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
}
