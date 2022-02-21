//! Error type for Fennel rpc connections

use thiserror::Error;
use subxt::BasicError as SubxtError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something happened with the internal node connection")]
    Subxt(#[from] SubxtError),
}
