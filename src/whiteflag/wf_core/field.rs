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

    /* fn is_valid(&self) {
        &self.pattern.is_match("")
    } */
}
