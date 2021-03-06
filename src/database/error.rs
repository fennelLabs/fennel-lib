//! Error type for Fennel Backend RocksDB

use rocksdb::Error as RocksDbError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error interacting with backend RocksDB")]
    Database(#[from] RocksDbError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Failed to acquire a handle to rocksdb column family cf{0}")]
    CfHandle(u32),
    #[error(transparent)]
    Codec(#[from] codec::Error),
}
