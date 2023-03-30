//! # The `database` module provides a simple API for interacting with a RocksDB database.
//!
//! This module contains the following main components:
//!
//! - [`RocksDB`]: A struct that provides a simple API for interacting with a RocksDB database.
//! - [`WasmFn`]: A struct representing a WebAssembly function.
//! - [`RocksDBError`]: An enumeration of potential errors that can be encountered while working with a RocksDB database.
//!
//! # Examples
//!
//! ```no_run
//! use database::RocksDB;
//! use database::models::WasmFn;
//!
//! let mut db = RocksDB::new();
//! let wasm = WasmFn::new("example_fn", "example.wasm");
//!
//! let key = "example_key";
//! let _ = db.add(key, wasm).unwrap();
//! let wasm_fn = db.get(key).unwrap();
//! println!("{:?}", wasm_fn);
//! let _ = db.del(key).unwrap();
//! ```

#![allow(dead_code)]

mod errors;
pub mod models;

use self::models::WasmFn;
use crate::logger;
use errors::RocksDBError;
use lazy_static::lazy_static;
use log::{error, info};
use rocksdb::{DBWithThreadMode, IteratorMode, MultiThreaded, Options, DB as DataBase};
use std::sync::{Arc, Mutex};

// Creating the single instance of RocksDB with inter-thread security.
lazy_static! {
    static ref DB: Arc<Mutex<DBWithThreadMode<MultiThreaded>>> = {
        let path = "./rocksdb/prod";
        let mut options = Options::default();
        options.create_if_missing(true);

        match DataBase::open_default(path) {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(err) => {
                error!(target: "err","DB dont open: {err}");
                panic!("DB dont open: {}", err);
            }
        }
    };
    static ref DEV_DB: Arc<Mutex<DBWithThreadMode<MultiThreaded>>> = {
        let path = "./rocksdb/dev";
        let mut options = Options::default();
        options.create_if_missing(true);

        match DataBase::open_default(path) {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(err) => {
                error!(target: "err","DEV DB dont open: {err}");
                panic!("DB dont open: {}", err);
            }
        }
    };
}

/// The `RocksDB` framework provides a simple API for interacting with the RocksDB database.
#[derive(Clone, Debug)]
pub struct RocksDB {
    db: Arc<Mutex<DBWithThreadMode<MultiThreaded>>>,
}

impl RocksDB {
    /// # Creates a new instance of the `RocksDB` structure.
    ///
    /// ## Returns
    ///
    /// * An instance of `RocksDB`.
    pub fn new() -> Self {
        RocksDB {
            db: Arc::clone(&DB),
        }
    }

    /// # [`TEST ONLY`]
    ///
    /// # Creates a new instance of the `RocksDB` structure
    /// which is used to interact with a temporary RocksDB database,
    /// used only for testing purposes.
    ///
    /// ## Arguments
    ///
    /// This function does not take any arguments.
    ///
    /// ## Returns
    ///
    /// * An instance of `RocksDB` - a structure that provides a simple API
    /// for interacting with a temporary RocksDB database used for testing purposes.
    pub fn dev() -> Self {
        RocksDB {
            db: Arc::clone(&DEV_DB),
        }
    }

    /// # Adds a new key-value pair to the RocksDB database.
    ///
    /// ## Arguments
    ///
    /// * `key` - A string slice that represents the key to be added.
    /// * `wasm` - A `WasmFn` object that represents the value to be added.
    ///
    /// ## Returns
    ///
    /// * A `Result` object that returns the key if the operation was successful,
    /// or a `RocksDBError` object if the operation failed.
    pub fn add(&mut self, key: &str, wasm: WasmFn) -> Result<String, RocksDBError> {
        info!(target: "tx", "CREATE {key}");
        self.db
            .lock()
            .unwrap()
            .put(key, serde_json::to_vec(&wasm).unwrap())
            .map_err(|e| logger::log_error(RocksDBError::Unknown(e.to_string())))
            .and_then(|_| Ok(key.to_string()))
    }

    /// # Gets the value of a key in the RocksDB database.
    ///
    /// ## Arguments
    ///
    /// * `key` - A string slice that represents the key to be retrieved.
    ///
    /// ## Returns
    ///
    /// * An `Option` that returns the value of the key if it exists in the database,
    /// or `None` if it doesn't.
    pub fn get(&self, key: &str) -> Option<WasmFn> {
        if let Some(v) = self
            .db
            .lock()
            .unwrap()
            .get(key)
            .map_err(|e| logger::log_error(RocksDBError::Unknown(e.to_string())))
            .unwrap_or_default()
        {
            Some(serde_json::from_slice::<WasmFn>(&v).unwrap())
        } else {
            None
        }
    }

    /// # Gets all key-value pairs from the RocksDB database.
    ///
    /// ## Returns
    ///
    /// * A `Vec` that contains all the key-value pairs in the database.
    /// Each element of the vector is an `Option` that returns the value if the key exists,
    /// or `None` if the key doesn't exist.
    pub fn all(&self) -> Vec<Option<WasmFn>> {
        self.db
            .lock()
            .unwrap()
            .iterator(IteratorMode::Start)
            .map(|item| match item {
                Ok((_, v)) => Some(serde_json::from_slice(&v).unwrap()),
                Err(e) => {
                    logger::log_error(RocksDBError::Unknown(e.to_string()));
                    None
                }
            })
            .collect()
    }

    /// # Updates the value of an existing key in the RocksDB database.
    ///
    /// ## Arguments
    ///
    /// * `key` - A string slice that represents the key to be updated.
    /// * `wasm` - A `WasmFn` object that represents the new value to be set.
    ///
    /// ## Returns
    ///
    /// * A `Result` object that returns the key if the operation was successful,
    /// or a `RocksDBError` object if the operation failed.
    ///
    /// # Errors
    ///
    /// Returns a `RocksDBError::NotFound` error if the key doesn't exist in the database.
    pub fn upd(&mut self, key: &str, wasm: WasmFn) -> Result<String, RocksDBError> {
        let value = self
            .db
            .lock()
            .unwrap()
            .get(key)
            .map_err(|e| logger::log_error(RocksDBError::Unknown(e.to_string())))
            .unwrap_or_default();
        info!(target: "tx", "UPDATE {key}");
        if let Some(_) = value {
            let new_value = serde_json::to_vec(&wasm).unwrap();
            self.db
                .lock()
                .unwrap()
                .put(key.as_bytes(), new_value)
                .map_err(|e| logger::log_error(RocksDBError::Unknown(e.to_string())))
                .unwrap();
            Ok(key.to_owned())
        } else {
            Err(logger::log_error(RocksDBError::NotFound))
        }
    }

    /// # Deletes a key from the RocksDB database.
    ///
    /// ## Arguments
    ///
    /// * `key` - A string slice that represents the key to be deleted.
    ///
    /// ## Returns
    ///
    /// * A `Result` object that returns the key if the operation was successful,
    /// or a `RocksDBError` object if the operation failed.
    ///
    /// # Errors
    ///
    /// Returns a `RocksDBError::NotFound` error if the key doesn't exist in the database.
    pub fn del(&mut self, key: &str) -> Result<String, RocksDBError> {
        info!(target: "tx", "DELETE {key}");
        let value = self
            .db
            .lock()
            .unwrap()
            .get(key)
            .map_err(|e| logger::log_error(RocksDBError::Unknown(e.to_string())))
            .unwrap();
        if let Some(_) = value {
            self.db
                .lock()
                .unwrap()
                .delete(key)
                .map_err(|e| logger::log_error(RocksDBError::Unknown(e.to_string())))
                .unwrap();
            Ok(key.to_owned())
        } else {
            Err(logger::log_error(RocksDBError::NotFound))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::WasmFn;

    #[test]
    fn test_add_and_get() {
        let mut db = RocksDB::dev();
        let wasm = WasmFn::default();
        let key = "example_key";

        let _ = db.add(key, wasm.clone()).unwrap();
        let wasm_fn = db.get(key).unwrap();

        assert_eq!(wasm_fn, wasm);
    }

    #[test]
    fn test_upd_and_del() {
        let mut db = RocksDB::dev();
        let wasm = WasmFn::default();
        let wasm_updated = WasmFn::default();
        let key = "example_key";

        let _ = db.add(key, wasm).unwrap();
        let _ = db.upd(key, wasm_updated.clone()).unwrap();
        let wasm_fn = db.get(key).unwrap();

        assert_eq!(wasm_fn, wasm_updated);

        let _ = db.del(key).unwrap();
        let wasm_fn = db.get(key);

        assert_eq!(wasm_fn, None);
    }
}
