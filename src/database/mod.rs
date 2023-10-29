//! # The `database` module provides a simple API for interacting with a RocksDB database.
//!
//! This module contains the following main components:
//!
//! - [`RocksDB`]: A struct that provides a simple API for interacting with a RocksDB database.
//! - [`WasmModule`]: A struct representing a WebAssembly function.
//!
//! # Examples
//!
//! ```no_run
//! use database::RocksDB;
//! use database::models::WasmModule;
//!
//! let mut db = RocksDB::new();
//! let wasm = WasmModule::new("example_fn", "example.wasm");
//!
//! let key = "example_key";
//! let _ = db.add(key, wasm).unwrap();
//! let wasm_module = db.get(key).unwrap();
//! println!("{:?}", wasm_module);
//! let _ = db.del(key).unwrap();
//! ```

#![allow(dead_code)]

pub mod models;

use self::models::WasmModule;
use crate::errors::WessError;
use crate::metrics::constants::DATABASE_OPERATIONS_TOTAL;
use crate::metrics::constants::DATABASE_OPERATION_DURATION;
use lazy_static::lazy_static;
use log::{error, info};
use rocksdb::{DBWithThreadMode, IteratorMode, MultiThreaded, Options, DB as DataBase};
use std::sync::{Arc, Mutex};
use std::time::Instant;

// Creating the single instance of RocksDB with inter-thread security.
lazy_static! {
    pub static ref DB: Arc<Mutex<DBWithThreadMode<MultiThreaded>>> = {
        let mut options = Options::default();
        options.create_if_missing(true);

        match DataBase::open_default("./rocksdb/prod") {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(err) => {
                error!(target: "wess::err","DB dont open: {err}");
                panic!("DB dont open: {}", err);
            }
        }
    };
    static ref DEV_DB: Arc<Mutex<DBWithThreadMode<MultiThreaded>>> = {
        let mut options = Options::default();
        options.create_if_missing(true);

        match DataBase::open_default("./rocksdb/dev") {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(err) => {
                error!(target: "wess::err","DEV DB dont open: {err}");
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
    /// * `wasm` - A `WasmModule` object that represents the value to be added.
    ///
    /// ## Returns
    ///
    /// * A `Result` object that returns the key if the operation was successful,
    /// or a `WessError` object if the operation failed.
    pub fn add(&mut self, key: &str, wasm: WasmModule) -> Result<String, WessError> {
        info!(target: "wess::tx", "CREATE {key}");
        DATABASE_OPERATIONS_TOTAL
            .with_label_values(&["write"])
            .inc();
        let start = Instant::now();

        let r = self
            .db
            .lock()
            .unwrap()
            .put(key, serde_json::to_vec(&wasm).unwrap())
            .map_err(|e| log_error!(e.to_string(), 500))
            .and_then(|_| Ok(key.to_string()));

        let duration = start.elapsed();
        DATABASE_OPERATION_DURATION
            .with_label_values(&["write"])
            .observe(duration.as_secs_f64());

        r
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
    pub fn get(&self, key: &str) -> Option<WasmModule> {
        DATABASE_OPERATIONS_TOTAL.with_label_values(&["read"]).inc();
        let start = Instant::now();

        let r = self
            .db
            .lock()
            .unwrap()
            .get(key)
            .map_err(|e| log_error!(e.to_string(), 500))
            .unwrap_or_default();

        let duration = start.elapsed();
        DATABASE_OPERATION_DURATION
            .with_label_values(&["read"])
            .observe(duration.as_secs_f64());

        if let Some(v) = r {
            Some(serde_json::from_slice::<WasmModule>(&v).unwrap())
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
    pub fn all(&self) -> Vec<Option<WasmModule>> {
        DATABASE_OPERATIONS_TOTAL.with_label_values(&["read"]).inc();
        let start = Instant::now();

        let r = self
            .db
            .lock()
            .unwrap()
            .iterator(IteratorMode::Start)
            .map(|item| match item {
                Ok((_, v)) => Some(serde_json::from_slice(&v).unwrap()),
                Err(e) => {
                    log_error!(e.to_string(), 500);
                    None
                }
            })
            .collect();

        let duration = start.elapsed();
        DATABASE_OPERATION_DURATION
            .with_label_values(&["read"])
            .observe(duration.as_secs_f64());

        r
    }

    /// # Updates the value of an existing key in the RocksDB database.
    ///
    /// ## Arguments
    ///
    /// * `key` - A string slice that represents the key to be updated.
    /// * `wasm` - A `WasmModule` object that represents the new value to be set.
    ///
    /// ## Returns
    ///
    /// * A `Result` object that returns the key if the operation was successful,
    /// or a `WessError` object if the operation failed.
    ///
    /// # Errors
    ///
    /// Returns a `WessError::NotFound` error if the key doesn't exist in the database.
    pub fn upd(&mut self, key: &str, wasm: WasmModule) -> Result<String, WessError> {
        info!(target: "wess::tx", "UPDATE {key}");
        DATABASE_OPERATIONS_TOTAL
            .with_label_values(&["write"])
            .inc();
        let new_value = serde_json::to_vec(&wasm).unwrap();
        let start = Instant::now();

        self.db
            .lock()
            .unwrap()
            .put(key.as_bytes(), new_value)
            .map_err(|e| log_error!(e.to_string(), 500))
            .unwrap();

        let duration = start.elapsed();
        DATABASE_OPERATION_DURATION
            .with_label_values(&["write"])
            .observe(duration.as_secs_f64());

        Ok(key.to_owned())
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
    /// or a `WessError` object if the operation failed.
    ///
    /// # Errors
    ///
    /// Returns a `WessError::NotFound` error if the key doesn't exist in the database.
    pub fn del(&mut self, key: &str) -> Result<String, WessError> {
        info!(target: "wess::tx", "DELETE {key}");
        DATABASE_OPERATIONS_TOTAL
            .with_label_values(&["write"])
            .inc();
        let start = Instant::now();

        self.db
            .lock()
            .unwrap()
            .delete(key)
            .map_err(|e| log_error!(e.to_string(), 500))
            .unwrap();

        let duration = start.elapsed();
        DATABASE_OPERATION_DURATION
            .with_label_values(&["write"])
            .observe(duration.as_secs_f64());

        Ok(key.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::WasmModule;

    #[test]
    fn test_add_and_get() {
        let mut db = RocksDB::dev();
        let wasm = WasmModule::default();
        let key = "example_key";

        let _ = db.add(key, wasm.clone()).unwrap();
        let wasm_module = db.get(key).unwrap();

        assert_eq!(wasm_module, wasm);
    }

    #[test]
    fn test_upd_and_del() {
        let mut db = RocksDB::dev();
        let wasm = WasmModule::default();
        let wasm_updated = WasmModule::default();
        let key = "example_key";

        let _ = db.add(key, wasm).unwrap();
        let _ = db.upd(key, wasm_updated.clone()).unwrap();
        let wasm_module = db.get(key).unwrap();

        assert_eq!(wasm_module, wasm_updated);

        let _ = db.del(key).unwrap();
        let wasm_module = db.get(key);

        assert_eq!(wasm_module, None);
    }
}
