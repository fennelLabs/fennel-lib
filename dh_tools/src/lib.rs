#[cfg(test)]
mod tests;

use x25519_dalek::{PublicKey, SharedSecret, StaticSecret};

// Brainpool imports (RFC 5639 compliance)
use bp256::r1::{
    SecretKey as BrainpoolSecretKey,
    AffinePoint as BrainpoolAffine,
    ProjectivePoint as BrainpoolProjective,
};
use elliptic_curve::{
    sec1::{EncodedPoint, FromEncodedPoint, ToEncodedPoint},
};

// ========================================
// X25519 Functions (Legacy/Internal Use)
// ========================================

/// Generates a static secret for the current session, usable in Diffie-Hellman.
pub fn get_session_secret() -> StaticSecret {
    // x25519_dalek uses rand_core 0.6, so we use getrandom directly
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes).expect("RNG failure");
    StaticSecret::from(bytes)
}

/// Based on the user's static secret, generate a public key that can be used to generate encryption resources.
pub fn get_session_public_key(secret: &StaticSecret) -> PublicKey {
    PublicKey::from(secret)
}

/// Carries out the Diffie-Hellman handshake with another user's public key to generate a shared secret.
pub fn get_shared_secret(my_secret: StaticSecret, their_public: &PublicKey) -> SharedSecret {
    my_secret.diffie_hellman(their_public)
}

// ========================================
// BrainpoolP256r1 Functions (Whiteflag Compliance)
// ========================================

/// Generates a brainpoolP256r1 keypair for Whiteflag RFC 5639 compliance.
/// 
/// Returns: (private_key: 32 bytes, public_key: 33 bytes SEC1 compressed)
pub fn generate_brainpool_keypair() -> (Vec<u8>, Vec<u8>) {
    // Generate random bytes using getrandom to avoid rand_core version conflicts
    // Keep trying until we get a valid key (very rare to need more than one iteration)
    loop {
        let mut seed = [0u8; 32];
        getrandom::getrandom(&mut seed).expect("RNG failure");
        
        // Try to import secret key from random bytes
        if let Ok(secret_key) = BrainpoolSecretKey::from_bytes(&seed.into()) {
            // Derive public key
            let secret_scalar = secret_key.to_nonzero_scalar();
            let public_projective = BrainpoolProjective::GENERATOR * secret_scalar.as_ref();
            let public_affine = public_projective.to_affine();
            
            // Export private key (32 bytes)
            let private_bytes = secret_key.to_bytes().to_vec();
            
            // Export public key as SEC1 compressed (33 bytes: 0x02/0x03 prefix + x-coordinate)
            let encoded_point = public_affine.to_encoded_point(true); // true = compressed
            let public_bytes = encoded_point.as_bytes().to_vec();
            
            return (private_bytes, public_bytes);
        }
        // If invalid, loop and try again
    }
}

/// Imports a brainpoolP256r1 keypair from a 32-byte private key.
/// 
/// Returns: (private_key: 32 bytes, public_key: 33 bytes SEC1 compressed)
pub fn import_brainpool_keypair(private_key_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), String> {
    if private_key_bytes.len() != 32 {
        return Err("Private key must be exactly 32 bytes".to_string());
    }
    
    // Convert slice to array
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(private_key_bytes);
    
    // Import secret key
    let secret_key = BrainpoolSecretKey::from_bytes(&key_array.into())
        .map_err(|_| "Invalid private key bytes".to_string())?;
    
    // Derive public key
    let secret_scalar = secret_key.to_nonzero_scalar();
    let public_projective = BrainpoolProjective::GENERATOR * secret_scalar.as_ref();
    let public_affine = public_projective.to_affine();
    
    // Export keys
    let private_bytes = secret_key.to_bytes().to_vec();
    let encoded_point = public_affine.to_encoded_point(true);
    let public_bytes = encoded_point.as_bytes().to_vec();
    
    Ok((private_bytes, public_bytes))
}

/// Computes the ECDH shared secret using brainpoolP256r1.
/// 
/// Arguments:
/// - my_private_key: Your 32-byte private key
/// - their_public_key: Peer's 33-byte SEC1 compressed public key
/// 
/// Returns: 32-byte shared secret (x-coordinate of shared point)
pub fn compute_brainpool_shared_secret(
    my_private_key: &[u8],
    their_public_key: &[u8],
) -> Result<Vec<u8>, String> {
    // Import my secret key
    if my_private_key.len() != 32 {
        return Err("Private key must be exactly 32 bytes".to_string());
    }
    
    // Convert slice to array
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(my_private_key);
    
    let secret_key = BrainpoolSecretKey::from_bytes(&key_array.into())
        .map_err(|_| "Invalid private key".to_string())?;
    
    // Parse their public key (SEC1 compressed)
    if their_public_key.len() != 33 {
        return Err("Public key must be exactly 33 bytes (SEC1 compressed)".to_string());
    }
    
    let encoded_point = EncodedPoint::<bp256::BrainpoolP256r1>::from_bytes(their_public_key)
        .map_err(|_| "Invalid SEC1 encoding".to_string())?;
    
    let their_affine = BrainpoolAffine::from_encoded_point(&encoded_point);
    if their_affine.is_none().into() {
        return Err("Point not on curve".to_string());
    }
    let their_affine = their_affine.unwrap();
    
    // Perform ECDH: shared_point = my_scalar * their_public_point
    let secret_scalar = secret_key.to_nonzero_scalar();
    let shared_projective = BrainpoolProjective::from(their_affine) * secret_scalar.as_ref();
    let shared_affine = shared_projective.to_affine();
    
    // Extract x-coordinate as shared secret (32 bytes)
    let shared_encoded = shared_affine.to_encoded_point(false); // uncompressed for x-coordinate access
    let x_bytes = shared_encoded.x()
        .ok_or_else(|| "Failed to extract x-coordinate".to_string())?;
    
    Ok(x_bytes.to_vec())
}

/// Converts a brainpoolP256r1 public key to SEC1 compressed format (33 bytes).
/// 
/// Accepts either:
/// - 33 bytes (already compressed, returns as-is)
/// - 65 bytes (uncompressed, converts to compressed)
pub fn brainpool_public_key_to_compressed(public_key: &[u8]) -> Result<Vec<u8>, String> {
    match public_key.len() {
        33 => {
            // Already compressed, validate and return
            let encoded_point = EncodedPoint::<bp256::BrainpoolP256r1>::from_bytes(public_key)
                .map_err(|_| "Invalid SEC1 compressed format".to_string())?;
            
            let affine = BrainpoolAffine::from_encoded_point(&encoded_point);
            if affine.is_none().into() {
                return Err("Point not on curve".to_string());
            }
            
            Ok(public_key.to_vec())
        }
        65 => {
            // Uncompressed, convert to compressed
            let encoded_point = EncodedPoint::<bp256::BrainpoolP256r1>::from_bytes(public_key)
                .map_err(|_| "Invalid SEC1 uncompressed format".to_string())?;
            
            let affine = BrainpoolAffine::from_encoded_point(&encoded_point);
            if affine.is_none().into() {
                return Err("Point not on curve".to_string());
            }
            let affine = affine.unwrap();
            
            // Re-encode as compressed
            let compressed_point = affine.to_encoded_point(true);
            Ok(compressed_point.as_bytes().to_vec())
        }
        _ => Err(format!(
            "Invalid public key length: expected 33 (compressed) or 65 (uncompressed), got {}",
            public_key.len()
        )),
    }
}

