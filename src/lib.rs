pub mod aes_tools;
pub mod database;
pub mod dh_tools;
pub mod fennel;
pub mod rsa_tools;
pub mod whiteflag;

use codec::{Decode, Encode};

pub use crate::aes_tools::*;
pub use crate::database::*;
pub use crate::dh_tools::*;
pub use crate::fennel::*;
pub use crate::rsa_tools::*;
pub use crate::whiteflag::*;

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
    pub message: [u8; 1024],

    /// A signature used to verify that the message's sender is appropriately claimed.
    pub signature: [u8; 1024],

    /// Represents an RSA public key used to sign the message.
    pub public_key: [u8; 1038],

    /// If sending a message, represents the person receiving.
    pub recipient: [u8; 4],

    /// Used to indicate the encryption scheme used in the sent message.
    pub message_type: [u8; 1],
}
