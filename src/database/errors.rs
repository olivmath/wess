pub enum RocksDBError {
    Unknown,
    NotFound,
    // ..
}

impl ToString for RocksDBError {
    fn to_string(&self) -> String {
        match self {
            RocksDBError::NotFound => "NotFound".to_string(),
            RocksDBError::Unknown => "Unknown".to_string(),
            // ...
        }
    }
}