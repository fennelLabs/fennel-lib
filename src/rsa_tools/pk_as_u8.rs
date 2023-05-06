use rsa::{
    pkcs1::{
        FromRsaPrivateKey, FromRsaPublicKey, RsaPrivateKeyDocument, RsaPublicKeyDocument,
        ToRsaPrivateKey, ToRsaPublicKey,
    },
    RsaPrivateKey, RsaPublicKey,
};

pub struct FennelRSAPublicKey {
    pkcs1: RsaPublicKeyDocument,
    pub pk: RsaPublicKey,
}

pub struct FennelRSAPrivateKey {
    pub pkcs1: RsaPrivateKeyDocument,
    pub pk: RsaPrivateKey,
}

impl FennelRSAPrivateKey {
    pub fn new(pk: RsaPrivateKey) -> Result<Self, rsa::pkcs1::Error> {
        Ok(Self {
            pkcs1: pk.to_pkcs1_der()?,
            pk,
        })
    }

    /// import private key from bytes
    pub fn from_u8(private_key_binary: &[u8]) -> Result<Self, rsa::pkcs1::Error> {
        let private_key = RsaPrivateKey::from_pkcs1_der(private_key_binary)?;
        Self::new(private_key)
    }

    pub fn as_u8(&self) -> &[u8] {
        self.pkcs1.as_der()
    }
}

impl FennelRSAPublicKey {
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
