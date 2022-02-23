//! Error type for Fennel rpc connections

use subxt::BasicError as SubxtError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something happened with the internal node connection")]
    Subxt(#[from] SubxtError),
}
