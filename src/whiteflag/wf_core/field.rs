use super::{
    error::{WhiteflagError, WhiteflagResult},
    wf_codec::encoding::*,
};
use regex::Regex;

#[derive(Clone)]
pub struct Field {
    pub name: String,
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

    pub fn get_minimum_starting_position(&self) -> usize {
        if self.end_byte < 0 {
            return self.start_byte;
        }

        self.end_byte as usize
    }

    /* pub fn get(&self, data: Vec<String>) -> WhiteflagResult<String> {
        if data.len() < self.get_minimum_starting_position() {
            return Err(WhiteflagError::InvalidLength);
        }

        data[self.start_byte..self.end_byte as usize]
            .first()
            .ok_or(WhiteflagError::InvalidLength)
    } */

    /**
     * Sets the value of the message field if not already set
     * @param data the data representing the field value
     * @return TRUE if field value is set, FALSE if field already set or data is invalid
     */
    pub fn set(&mut self, data: String) -> WhiteflagResult<()> {
        if !self.pattern.is_match(&data) {
            return Err(WhiteflagError::InvalidPattern);
        }

        self.value = Some(data);
        Ok(())
    }

    pub fn get(&self) -> &Option<String> {
        &self.value
    }

    /**
     * Checks if the message field value has been set. FieldDefinition is considered set if it contains a valid value.
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
        let value = match &self.value {
            Some(x) => x,
            None => return false,
        };
        self.pattern.is_match(value)
    }
}
