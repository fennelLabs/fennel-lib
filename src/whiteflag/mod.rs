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
            message_code: message_code.clone(),
            message_type: message_code.clone(),
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
        self.encryption_indicator.clone()
    }
    pub fn set_encryption_indicator(&mut self, arg: String) -> bool {
        if self.encryption_indicator == "" {
            self.encryption_indicator = arg;
            return true;
        }
        return false;
    }

    pub fn get_subject_code(&self) -> String {
        self.subject_code.clone()
    }
    pub fn set_subject_code(&mut self, arg: String) -> bool {
        if self.subject_code == "" {
            self.subject_code = arg;
            return true;
        }
        return false;}

    pub fn get_object_type(&self) -> String {
        self.object_type.clone()
    }
    pub fn set_object_type(&mut self, arg: String) -> bool {
        if self.object_type == "" {
            self.object_type = arg;
            return true;
        }
        return false;}

    pub fn get_transaction_hash(&self) -> String {
        self.transaction_hash.clone()
    }
    pub fn set_transaction_hash(&mut self, arg: String) -> Option<String> {
        if self.transaction_hash == "" {
            self.transaction_hash = arg;
            return None;
        }
        Some(self.transaction_hash.clone())
    }

    pub fn get_originator_address(&self) -> String {
        self.originator_address.clone()
    }
    pub fn set_originator_address(&mut self, arg: String) -> Option<String> {
        if self.originator_address == "" {
            self.originator_address = arg;
            return None;
        }
        Some(self.originator_address.clone())
    }

    /// Set the whiteflag message's message type.
    pub fn set_message_type(&mut self, message_type: String) {
        self.message_type = message_type;
    }

    /// Get a reference to the whiteflag message's prefix.
    pub fn prefix(&self) -> &str {
        self.prefix.as_ref()
    }

    /// Get a reference to the whiteflag message's version.
    pub fn version(&self) -> &str {
        self.version.as_ref()
    }

    /// Get a mutable reference to the whiteflag message's duress indictor.
    pub fn duress_indictor_mut(&mut self) -> &mut String {
        &mut self.duress_indictor
    }

    /// Get a mutable reference to the whiteflag message's message code.
    pub fn message_code_mut(&mut self) -> &mut String {
        &mut self.message_code
    }

    /// Get a mutable reference to the whiteflag message's reference indicator.
    pub fn reference_indicator_mut(&mut self) -> &mut String {
        &mut self.reference_indicator
    }

    /// Get a mutable reference to the whiteflag message's referenced message.
    pub fn referenced_message_mut(&mut self) -> &mut String {
        &mut self.referenced_message
    }

    /// Get a reference to the whiteflag message's crypto data type.
    pub fn crypto_data_type(&self) -> &str {
        self.crypto_data_type.as_ref()
    }

    /// Get a mutable reference to the whiteflag message's crypto data.
    pub fn crypto_data_mut(&mut self) -> &mut String {
        &mut self.crypto_data
    }

    /// Get a mutable reference to the whiteflag message's crypto data type.
    pub fn crypto_data_type_mut(&mut self) -> &mut String {
        &mut self.crypto_data_type
    }

    /// Set the whiteflag message's crypto data type.
    pub fn set_crypto_data_type(&mut self, crypto_data_type: String) {
        self.crypto_data_type = crypto_data_type;
    }

    /// Get a reference to the whiteflag message's crypto data.
    pub fn crypto_data(&self) -> &str {
        self.crypto_data.as_ref()
    }

    /// Set the whiteflag message's crypto data.
    pub fn set_crypto_data(&mut self, crypto_data: String) {
        self.crypto_data = crypto_data;
    }

    /// Get a mutable reference to the whiteflag message's prefix.
    pub fn prefix_mut(&mut self) -> &mut String {
        &mut self.prefix
    }

    /// Get a reference to the whiteflag message's message type.
    pub fn message_type(&self) -> &str {
        self.message_type.as_ref()
    }

    /// Get a mutable reference to the whiteflag message's message type.
    pub fn message_type_mut(&mut self) -> &mut String {
        &mut self.message_type
    }
}