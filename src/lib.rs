#![feature(test)]

pub mod aes_tools;
pub mod database;
pub mod rsa_tools;

#[derive(Copy, Clone)]
pub struct FennelServerPacket {
    command: [u8; 1],
    identity: [u8; 32],
    fingerprint: [u8; 32],
    message: [u8; 1024],
    signature: [u8; 1024],
    public_key: [u8; 1038],
    recipient: [u8; 32],
}
