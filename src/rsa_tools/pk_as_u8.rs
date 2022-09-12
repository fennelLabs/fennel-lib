use rsa::{
    pkcs1::{FromRsaPublicKey, RsaPublicKeyDocument, ToRsaPublicKey},
    RsaPublicKey,
};

pub struct FennelRSAKeyPair {
    pkcs1: RsaPublicKeyDocument,
    pub pk: RsaPublicKey,
}

impl FennelRSAKeyPair {
    pub fn new(pk: RsaPublicKey) -> Result<Self, rsa::pkcs1::Error> {
        Ok(Self {
            pkcs1: pk.to_pkcs1_der()?,
            pk,
        })
    }

    /// import public key from bytes
    pub fn from_u8(public_key_binary: &[u8]) -> Result<Self, rsa::pkcs1::Error> {
        let public_key = RsaPublicKey::from_pkcs1_der(public_key_binary)?;
        Self::new(public_key)
    }

    pub fn as_u8(&self) -> &[u8] {
        self.pkcs1.as_der()
    }
}
