/// An enum representing possible errors that can occur while interacting with RocksDB.
#[derive(Debug)]
pub enum RocksDBError {
    /// An unknown error occurred.
    Unknown(String),
    /// The requested key was not found in the database.
    NotFound,
    // ...
}

impl ToString for RocksDBError {
    /// Converts the error into a string representation.
    fn to_string(&self) -> String {
        match self {
            RocksDBError::NotFound => "NotFound".to_string(),
            RocksDBError::Unknown(e) => format!("Unknown {}", e),
            // ...
        }
    }
}
