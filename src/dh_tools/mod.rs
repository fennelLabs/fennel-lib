#[cfg(test)]
mod tests;

use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

pub fn get_ephemeral_secret() -> EphemeralSecret {
    EphemeralSecret::new(OsRng)
}

pub fn get_ephemeral_public_key(secret: &EphemeralSecret) -> PublicKey {
    PublicKey::from(secret)
}

pub fn get_shared_secret(my_secret: EphemeralSecret, their_public: &PublicKey) -> SharedSecret {
    my_secret.diffie_hellman(their_public)
}