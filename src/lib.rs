pub mod fennel;
pub mod ipfs;
pub mod rsa_tools;

use codec::{Decode, Encode};

pub use crate::fennel::*;
pub use crate::rsa_tools::*;
pub use aes_tools::*;
pub use dh_tools::*;
pub use ipfs::*;

/// The FennelServerPacket struct is used for carrying formatted messages between the server and client.
#[derive(Copy, Clone, Encode, Decode, Debug)]
pub struct FennelServerPacket {
    /// A code corresponding to the command being sent to the server.
    pub command: [u8; 1],

    /// Indicates either the identity taking the action or the identity referenced by the action.
    pub identity: [u8; 4],

    /// If a public key is involved in the packet, represents that key's fingerprint.
    pub fingerprint: [u8; 16],

    /// If a message is being sent, this field represents the body of that message.
    pub message: [u8; 512],

    /// A signature used to verify that the message's sender is appropriately claimed.
    pub signature: [u8; 512],

    /// Represents an RSA public key used to sign the message.
    pub public_key: [u8; 526],

    /// If sending a message, represents the person receiving.
    pub recipient: [u8; 4],

    /// Used to indicate the encryption scheme used in the sent message.
    pub message_type: [u8; 1],
}
