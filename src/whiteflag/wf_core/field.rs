use super::wf_codec::encoding::*;
use regex::Regex;

#[derive(Clone)]
pub struct Field {
    name: String,
    pattern: Regex,
    encoding: Encoding,
    start_byte: usize,
    end_byte: isize,
    value: Option<String>,
}

impl Field {
    pub fn new(
        name: &str,
        pattern: Option<Regex>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> Field {
        Field {
            name: String::from(name),
            pattern: pattern.expect("invalid regular expression pattern"),
            encoding,
            start_byte,
            end_byte,
            value: None,
        }
    }

    /**
     * Sets the value of the message field if not already set
     * @param data the data representing the field value
     * @return TRUE if field value is set, FALSE if field already set or data is invalid
     */
    pub fn set(&mut self, data: String) -> bool {
        /* Cannot set value twice */
        if self.is_set() {
            return false;
        }

        /* Set if data is valid */
        if self.pattern.is_match(&data) {
            self.value = Some(data);
            return true;
        }

        false
    }

    /* fn is_valid(&self, data: &str) -> bool {
        self.pattern.is_match(data)
    } */

    /**
     * Checks if the message field value has been set. Field is considered set if it contains a valid value.
     * @return TRUE if the field has been set, else FALSE
     */
    pub fn is_set(&self) -> bool {
        self.is_valid()
    }

    /**
     * Checks if the message field contains a valid value
     * @return TRUE if the field contains a valid value, else FALSE
     */
    pub fn is_valid(&self) -> bool {
        let value = self.value.as_ref().unwrap();
        self.pattern.is_match(value)
    }
}
