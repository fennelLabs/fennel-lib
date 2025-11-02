use crate::{
    brainpool_public_key_to_compressed, compute_brainpool_shared_secret,
    generate_brainpool_keypair, get_session_public_key, get_session_secret, get_shared_secret,
    import_brainpool_keypair,
};
use aes_tools::{AESCipher, FennelCipher};

// ========================================
// X25519 Tests (Legacy)
// ========================================

#[test]
fn try_generating_key_and_encrypting() {
    let secret = get_session_secret();
    let pub_key = get_session_public_key(&secret);

    let other_secret = get_session_secret();
    let other_pub_key = get_session_public_key(&other_secret);

    let shared_secret = get_shared_secret(secret, &other_pub_key);
    let other_shared_secret = get_shared_secret(other_secret, &pub_key);

    assert_eq!(shared_secret.as_bytes(), other_shared_secret.as_bytes());

    let cipher: AESCipher = AESCipher::new_from_shared_secret(shared_secret.as_bytes());

    let ciphertext = cipher.encrypt("This is a test.");
    let plaintext = cipher.decrypt(ciphertext);

    assert_eq!("This is a test.", String::from_utf8_lossy(&plaintext));
}

// ========================================
// BrainpoolP256r1 Tests (Whiteflag Compliance)
// ========================================

#[test]
fn test_brainpool_keypair_generation() {
    let (private_key, public_key) = generate_brainpool_keypair();

    // Verify key sizes
    assert_eq!(private_key.len(), 32, "Private key should be 32 bytes");
    assert_eq!(
        public_key.len(),
        33,
        "Public key should be 33 bytes (SEC1 compressed)"
    );

    // Verify SEC1 compressed prefix (0x02 or 0x03)
    assert!(
        public_key[0] == 0x02 || public_key[0] == 0x03,
        "Public key should start with 0x02 or 0x03 (SEC1 compressed)"
    );
}

#[test]
fn test_brainpool_keypair_import() {
    // Generate a keypair
    let (private_key, public_key) = generate_brainpool_keypair();

    // Re-import from private key
    let (reimported_private, reimported_public) =
        import_brainpool_keypair(&private_key).expect("Failed to import keypair");

    // Verify keys match
    assert_eq!(private_key, reimported_private, "Private keys should match");
    assert_eq!(public_key, reimported_public, "Public keys should match");
}

#[test]
fn test_brainpool_ecdh_shared_secret() {
    // Alice generates keypair
    let (alice_private, alice_public) = generate_brainpool_keypair();

    // Bob generates keypair
    let (bob_private, bob_public) = generate_brainpool_keypair();

    // Alice computes shared secret using Bob's public key
    let alice_shared = compute_brainpool_shared_secret(&alice_private, &bob_public)
        .expect("Alice failed to compute shared secret");

    // Bob computes shared secret using Alice's public key
    let bob_shared = compute_brainpool_shared_secret(&bob_private, &alice_public)
        .expect("Bob failed to compute shared secret");

    // Shared secrets should match
    assert_eq!(alice_shared, bob_shared, "Shared secrets should match");
    assert_eq!(alice_shared.len(), 32, "Shared secret should be 32 bytes");
}

#[test]
fn test_brainpool_ecdh_with_encryption() {
    // Generate two keypairs
    let (private1, public1) = generate_brainpool_keypair();
    let (private2, public2) = generate_brainpool_keypair();

    // Compute shared secrets
    let shared1 = compute_brainpool_shared_secret(&private1, &public2)
        .expect("Failed to compute shared secret 1");
    let shared2 = compute_brainpool_shared_secret(&private2, &public1)
        .expect("Failed to compute shared secret 2");

    assert_eq!(shared1, shared2, "Shared secrets must match");
    assert_eq!(shared1.len(), 32, "Shared secret must be 32 bytes");

    // Use shared secret for AES encryption - convert Vec to array
    let shared_array: [u8; 32] = shared1
        .as_slice()
        .try_into()
        .expect("Shared secret not 32 bytes");
    let cipher = AESCipher::new_from_shared_secret(&shared_array);

    let plaintext = "Whiteflag brainpool test message";
    let ciphertext = cipher.encrypt(plaintext);
    let decrypted = cipher.decrypt(ciphertext);

    assert_eq!(plaintext, String::from_utf8_lossy(&decrypted));
}

#[test]
fn test_brainpool_public_key_compression() {
    let (_, public_key) = generate_brainpool_keypair();

    // Compressed key should remain unchanged
    let compressed =
        brainpool_public_key_to_compressed(&public_key).expect("Failed to process compressed key");

    assert_eq!(
        public_key, compressed,
        "Compressed key should remain unchanged"
    );
    assert_eq!(compressed.len(), 33, "Compressed key should be 33 bytes");
}

#[test]
fn test_brainpool_deterministic_keys() {
    // Use same private key bytes twice
    let seed = [0x42u8; 32];

    let (priv1, pub1) = import_brainpool_keypair(&seed).expect("Failed to import keypair 1");
    let (priv2, pub2) = import_brainpool_keypair(&seed).expect("Failed to import keypair 2");

    // Keys should be identical
    assert_eq!(priv1, priv2, "Private keys should be deterministic");
    assert_eq!(pub1, pub2, "Public keys should be deterministic");
}

#[test]
fn test_brainpool_invalid_inputs() {
    // Test invalid private key length
    let bad_private = vec![0u8; 16]; // Wrong size
    assert!(import_brainpool_keypair(&bad_private).is_err());

    // Test invalid public key length
    let (private, _) = generate_brainpool_keypair();
    let bad_public = vec![0u8; 32]; // Wrong size (should be 33)
    assert!(compute_brainpool_shared_secret(&private, &bad_public).is_err());

    // Test invalid SEC1 encoding
    let bad_sec1 = vec![0x04; 33]; // Wrong prefix (should be 0x02 or 0x03)
    assert!(compute_brainpool_shared_secret(&private, &bad_sec1).is_err());
}
