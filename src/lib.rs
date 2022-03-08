#![feature(test)]

pub mod aes_tools;
pub mod database;
pub mod dh_tools;
pub mod fennel;
pub mod rsa_tools;

use codec::{Decode, Encode};

pub use crate::aes_tools::*;
pub use crate::database::*;
pub use crate::fennel::*;
pub use crate::dh_tools::*;
pub use crate::rsa_tools::*;

#[derive(Copy, Clone, Encode, Decode, Debug)]
pub struct FennelServerPacket {
    pub command: [u8; 1],
    pub identity: [u8; 4],
    pub fingerprint: [u8; 16],
    pub message: [u8; 1024],
    pub signature: [u8; 1024],
    pub public_key: [u8; 1038],
    pub recipient: [u8; 4],
}
