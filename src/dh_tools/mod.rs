#[cfg(test)]
mod tests;

use rand_core::OsRng;
use x25519_dalek::{StaticSecret, PublicKey, SharedSecret};

pub fn get_ephemeral_secret() -> StaticSecret {
    StaticSecret::new(OsRng)
}

pub fn get_ephemeral_public_key(secret: &StaticSecret) -> PublicKey {
    PublicKey::from(secret)
}

pub fn get_shared_secret(my_secret: StaticSecret, their_public: &PublicKey) -> SharedSecret {
    my_secret.diffie_hellman(their_public)
}
