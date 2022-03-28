use super::wf_codec::encoding::*;
use regex::Regex;

#[derive(Clone)]
pub struct FieldDefinition {
    name: String,
    pattern: Regex,
    encoding: Encoding,
    start_byte: usize,
    end_byte: isize,
}

pub struct Field {
    definition: FieldDefinition,
    value: String,
}

impl FieldDefinition {
    pub fn new(
        name: &str,
        pattern: Option<Regex>,
        encoding: Encoding,
        start_byte: usize,
        end_byte: isize,
    ) -> FieldDefinition {
        FieldDefinition {
            name: String::from(name),
            pattern: pattern.expect("invalid regular expression pattern"),
            encoding,
            start_byte,
            end_byte,
        }
    }
}

#[derive(Debug)]
pub enum FieldError {
    PatternDoesNotMatch,
}

pub type FieldResult = Result<Field, FieldError>;

impl Field {
    /**
     * Sets the value of the message field if not already set
     * @param data the data representing the field value
     * @return TRUE if field value is set, FALSE if field already set or data is invalid
     */
    pub fn set(definition: FieldDefinition, data: String) -> FieldResult {
        if !definition.pattern.is_match(&data) {
            return Err(FieldError::PatternDoesNotMatch);
        }

        Ok(Field {
            definition,
            value: data,
        })
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
        self.definition.pattern.is_match(&self.value)
    }
}
