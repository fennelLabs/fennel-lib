use crate::rsa_tools::pk_as_u8::FennelRSAKeyPair;

use super::*;
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    static ref KEYPAIR: (RsaPrivateKey, RsaPublicKey) = generate_keypair(2048);
    static ref KEYPAIR_4096: (RsaPrivateKey, RsaPublicKey) = generate_keypair(2048);
}

#[test]
fn test_export() {
    let (private_key, public_key) = (&KEYPAIR.0, &KEYPAIR.1);
    export_keypair_to_file(
        private_key,
        public_key,
        PathBuf::from("./Private.key"),
        PathBuf::from("./Public.key"),
    )
    .expect("failed to export keys");
}

#[test]
fn test_import() {
    let (private_key, public_key) = (&KEYPAIR.0, &KEYPAIR.1);
    export_keypair_to_file(
        private_key,
        public_key,
        PathBuf::from("./Private.key"),
        PathBuf::from("./Public.key"),
    )
    .expect("failed to export keys");
    let (new_private_key, new_public_key) = import_keypair_from_file(
        PathBuf::from("./Private.key"),
        PathBuf::from("./Public.key"),
    )
    .expect("failed to import key");
    assert_eq!(private_key, &new_private_key);
    assert_eq!(public_key, &new_public_key);
}

#[test]
fn test_encrypt() {
    let test = b"this is test text";
    let public_key = &KEYPAIR.1;
    encrypt(public_key, test.to_vec());
}

#[test]
fn test_decrypt() {
    let test = b"this is test text";
    let (private_key, public_key) = (&KEYPAIR.0, &KEYPAIR.1);
    let result = encrypt(public_key, test.to_vec());
    let decrypt_result = decrypt(private_key, result);
    assert_eq!(test.to_vec(), decrypt_result);
}

#[test]
fn test_sign() {
    let test = b"this is test text";
    let private_key = &KEYPAIR.0;
    sign(private_key, test.to_vec());
}

#[test]
fn test_verify() {
    let test = b"this is test text";
    let (private_key, public_key) = (&KEYPAIR.0, &KEYPAIR.1);
    let signed = sign(private_key, test.to_vec());
    verify(public_key, test.to_vec(), signed);
}

#[test]
fn test_import_public_key_from_binary() {
    let public_key = &KEYPAIR_4096.1;
    let pk = FennelRSAKeyPair::new(public_key.clone()).expect("failed to decode public key");
    let key_bytes = pk.as_u8();
    let new_key = FennelRSAKeyPair::from_u8(key_bytes).expect("failed to encode public key");
    assert_eq!(public_key, &new_key.pk);
}
