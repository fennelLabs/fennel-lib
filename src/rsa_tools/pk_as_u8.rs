use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    RsaPrivateKey, RsaPublicKey,
};

pub struct FennelRSAPublicKey {
    pkcs1: Vec<u8>,
    pub pk: RsaPublicKey,
}

pub struct FennelRSAPrivateKey {
    pub pkcs1: Vec<u8>,
    pub pk: RsaPrivateKey,
}

impl FennelRSAPrivateKey {
    pub fn new(pk: RsaPrivateKey) -> Result<Self, rsa::pkcs1::Error> {
        let doc = pk.to_pkcs1_der()?;
        let pkcs1 = doc.as_bytes().to_vec();
        Ok(Self { pkcs1, pk })
    }

    /// import private key from bytes
    pub fn from_u8(private_key_binary: &[u8]) -> Result<Self, rsa::pkcs1::Error> {
        let private_key = RsaPrivateKey::from_pkcs1_der(private_key_binary)?;
        Self::new(private_key)
    }

    pub fn as_u8(&self) -> &[u8] {
        &self.pkcs1
    }
}

impl FennelRSAPublicKey {
    pub fn new(pk: RsaPublicKey) -> Result<Self, rsa::pkcs1::Error> {
        let doc = pk.to_pkcs1_der()?;
        let pkcs1 = doc.as_bytes().to_vec();
        Ok(Self { pkcs1, pk })
    }

    /// import public key from bytes
    pub fn from_u8(public_key_binary: &[u8]) -> Result<Self, rsa::pkcs1::Error> {
        let public_key = RsaPublicKey::from_pkcs1_der(public_key_binary)?;
        Self::new(public_key)
    }

    pub fn as_u8(&self) -> &[u8] {
        &self.pkcs1
    }
}
