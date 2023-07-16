#[cfg(test)]
mod tests;

use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

/// Generates a static secret for the current session, usable in Diffie-Hellman.
pub fn get_session_secret() -> EphemeralSecret {
    EphemeralSecret::random_from_rng(OsRng)
}

/// Based on the user's static secret, generate a public key that can be used to generate encryption resources.
pub fn get_session_public_key(secret: &EphemeralSecret) -> PublicKey {
    PublicKey::from(secret)
}

/// Carries out the Diffie-Hellman handshake with another user's public key to generate a shared secret.
pub fn get_shared_secret(my_secret: EphemeralSecret, their_public: &PublicKey) -> SharedSecret {
    my_secret.diffie_hellman(their_public)
}
