use std::fmt;

/// An enum representing possible errors that can occur while interacting with RocksDB.
#[derive(Debug)]
pub enum RocksDBError {
    /// An unknown error occurred.
    Unknown(String),
    /// The requested key was not found in the database.
    NotFound,
    // ...
}

impl fmt::Display for RocksDBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RocksDBError::Unknown(message) => write!(f, "Unknown error occurred: {}", message),
            RocksDBError::NotFound => write!(f, "Requested key not found in the database"),
            // ...
        }
    }
}
