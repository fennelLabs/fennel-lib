#[cfg(test)]
mod tests;

#[derive(Clone)]
pub struct WhiteflagMessage {
    prefix: String,
    message_type: String,
    version: String,
    message_code: String,

    duress_indictor: String,
    encryption_indicator: String,
    object_type: String,
    subject_code: String,

    reference_indicator: String,
    referenced_message: String,

    crypto_data_type: String,
    crypto_data: String,

    transaction_hash: String,
    originator_address: String,
}

impl WhiteflagMessage {
    pub fn new(message_code: String) -> WhiteflagMessage {
        WhiteflagMessage {
            prefix: "WF".to_string(),
            version: "1".to_string(),
            message_code: message_code,
            message_type: "".to_string(),
            duress_indictor: "".to_string(),
            encryption_indicator: "".to_string(),
            object_type: "".to_string(),
            subject_code: message_code,
            reference_indicator: "".to_string(),
            referenced_message: "".to_string(),
            crypto_data_type: "".to_string(),
            crypto_data: "".to_string(),
            transaction_hash: "".to_string(),
            originator_address: "".to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        false
    }

    pub fn compile(&self) {}

    pub fn get_encryption_indicator(&self) -> String {
        self.encryption_indicator
    }
    pub fn set_encryption_indicator(&self, arg: String) -> bool {}

    pub fn get_subject_code(&self) -> String {
        self.subject_code
    }
    pub fn set_subject_code(&self, arg: String) -> bool {}

    pub fn get_object_type(&self) -> String {
        self.object_type
    }
    pub fn set_object_type(&self, arg: String) -> bool {}

    pub fn get_transaction_hash(&self) -> String {
        self.transaction_hash
    }
    pub fn set_transaction_hash(&self, arg: String) -> Option<String> {}

    pub fn get_originator_address(&self) -> String {
        self.originator_address
    }
    pub fn set_originator_address(&self, arg: String) -> Option<String> {}
}