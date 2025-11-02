#[cfg(test)]
mod tests;

use rand::rngs::OsRng;
use rsa::pkcs1v15::Pkcs1v15Encrypt;
use rsa::pkcs8::{Error, LineEnding};
use rsa::signature::{SignatureEncoding, Signer, Verifier};
pub use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use sha2::{Digest as Sha2Digest, Sha256};
use sha3::{Digest as Sha3Digest, Sha3_512};
use std::hash::Hash;

mod pk_as_u8;

pub use pk_as_u8::FennelRSAPrivateKey;
pub use pk_as_u8::FennelRSAPublicKey;

pub fn hash<H: Hash + AsRef<[u8]>>(text: H) -> Vec<u8> {
    let mut hasher = Sha3_512::new();
    Sha3Digest::update(&mut hasher, text);
    Sha3Digest::finalize(hasher).to_vec()
}

/// Generate a public/private keypair and return it as RSA structs.
#[allow(unused)]
pub fn generate_keypair(bits: usize) -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

/// Given plaintext, encrypt it with the provided public key.
#[allow(unused)]
pub fn encrypt(public_key: &RsaPublicKey, plaintext: Vec<u8>) -> Vec<u8> {
    let mut rng = OsRng;
    let padding = Pkcs1v15Encrypt;
    public_key
        .encrypt(&mut rng, padding, &plaintext[..])
        .expect("failed to encrypt")
}

/// Given a private key, decrypt ciphertext produced with its related public key.
#[allow(unused)]
pub fn decrypt(private_key: &RsaPrivateKey, ciphertext: Vec<u8>) -> Vec<u8> {
    let padding = Pkcs1v15Encrypt;
    private_key
        .decrypt(padding, &ciphertext)
        .expect("failed to decrypt")
}

// Issue a signature from `private_key` on `message`.
#[allow(unused)]
pub fn sign(private_key: &RsaPrivateKey, message: Vec<u8>) -> Vec<u8> {
    let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new_unprefixed(private_key.clone());
    let digest = hash(&message);
    signing_key.sign(&digest).to_bytes().to_vec()
}

/// Verify that a signature for a message is valid.
pub fn verify(public_key: &RsaPublicKey, message: Vec<u8>, signature: Vec<u8>) -> bool {
    let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new_unprefixed(public_key.clone());
    let digest = hash(&message);
    verifying_key
        .verify(
            &digest,
            &rsa::pkcs1v15::Signature::try_from(signature.as_slice()).unwrap(),
        )
        .is_ok()
}

/// Read in a keypair from a file.
#[allow(unused)]
pub fn import_keypair_from_file(
    private_keyfile_path: std::path::PathBuf,
    public_keyfile_path: std::path::PathBuf,
) -> Result<(RsaPrivateKey, RsaPublicKey), Error> {
    let pri = RsaPrivateKey::read_pkcs8_pem_file(private_keyfile_path)?;
    let pbk = RsaPublicKey::read_public_key_pem_file(public_keyfile_path)?;
    Ok((pri, pbk))
}

/// Write an in-memory keypair out to a file.
#[allow(unused)]
pub fn export_keypair_to_file(
    private_key: &RsaPrivateKey,
    public_key: &RsaPublicKey,
    private_keyfile_path: std::path::PathBuf,
    public_keyfile_path: std::path::PathBuf,
) -> Result<(), Error> {
    RsaPrivateKey::write_pkcs8_pem_file(private_key, private_keyfile_path, LineEnding::LF)?;
    RsaPublicKey::write_public_key_pem_file(public_key, public_keyfile_path, LineEnding::LF)?;
    Ok(())
}
