//! The `database` module provides a simple API for interacting with a RocksDB database.

#![allow(dead_code)]

use super::wasm::Wasm;
use lazy_static::lazy_static;
use rocksdb::{DBWithThreadMode, Error, IteratorMode, MultiThreaded, Options, DB as DataBase};
use std::sync::{Arc, Mutex};

// Creating the single instance of RocksDB with inter-thread security.
lazy_static! {
    static ref DB: Arc<Mutex<DBWithThreadMode<MultiThreaded>>> = {
        let path = "./rocksdb/prod";
        let mut options = Options::default();
        options.create_if_missing(true);

        match DataBase::open_default(path) {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(err) => panic!("DB dont open: {}", err),
        }
    };
    static ref DEV_DB: Arc<Mutex<DBWithThreadMode<MultiThreaded>>> = {
        let path = "./rocksdb/dev";
        let mut options = Options::default();
        options.create_if_missing(true);

        match DataBase::open_default(path) {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(err) => panic!("DB dont open: {}", err),
        }
    };
}

/// The `RocksDB` framework provides a simple API for interacting with the RocksDB database.
pub struct RocksDB {
    db: Arc<Mutex<DBWithThreadMode<MultiThreaded>>>,
}

impl RocksDB {
    /// Creates a new instance of the `RocksDB` structure.
    ///
    /// # Returns
    ///
    /// * An instance of `RocksDB`.
    pub fn new() -> Self {
        RocksDB {
            db: Arc::clone(&DB),
        }
    }

    /// # [`TEST ONLY`]
    ///
    /// Creates a new instance of the `RocksDB` structure, which is used to interact with a temporary RocksDB database, used only for testing purposes.
    ///
    /// # Arguments
    ///
    /// This function does not take any arguments.
    ///
    /// # Returns
    ///
    /// * An instance of `RocksDB` - a structure that provides a simple API for interacting with a temporary RocksDB database used for testing purposes.
    pub fn dev() -> Self {
        RocksDB {
            db: Arc::clone(&DEV_DB),
        }
    }

    /// Puts a `Wasm` instance into the database under the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that represents the key of the `Wasm` instance.
    /// * `wasm` - A `Wasm` instance to be stored in the database.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub fn put(&mut self, key: &str, wasm: Wasm) -> Result<(), Error> {
        let value = serde_json::to_vec(&wasm).unwrap();
        self.db.lock().unwrap().put(key.as_bytes(), value)
    }

    /// Retrieves a `Wasm` instance from the database by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that represents the key of the `Wasm` instance.
    ///
    /// # Returns
    ///
    /// An `Option` that wraps the `Wasm` instance if it is found in the database.
    pub fn get(&self, key: &str) -> Option<Wasm> {
        match self.db.lock().unwrap().get(key.as_bytes()).unwrap() {
            Some(v) => serde_json::from_slice(&v).unwrap(),
            None => None,
        }
    }

    /// Deletes a `Wasm` instance from the database by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that represents the key of the `Wasm` instance.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub fn del(&mut self, key: &str) -> Result<(), Error> {
        self.db.lock().unwrap().delete(key.as_bytes())
    }

    /// Retrieves all `Wasm` instances from the database.
    ///
    /// # Returns
    ///
    /// A `Vec` that contains `Option<Wasm>` elements. Each element is either `Some(Wasm)`
    /// if a `Wasm` instance was found in the database, or `None` if an error occurred.
    pub fn get_all(&self) -> Vec<Option<Wasm>> {
        self.db
            .lock()
            .unwrap()
            .iterator(IteratorMode::Start)
            .map(|item| match item {
                Ok((_, v)) => Some(serde_json::from_slice(&v).unwrap()),
                Err(_) => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::{Wasm, WasmFnArgs, WasmMetadata};

    fn wasm_mocked() -> Wasm {
        Wasm {
            wasm: vec![1, 2, 3],
            metadata: WasmMetadata {
                owner: vec![4, 5, 6],
                signature: vec![7, 8, 9],
                id: 123,
                fn_main: "main".to_string(),
                args: vec![
                    WasmFnArgs {
                        value: serde_json::from_str("{}").unwrap(),
                        name: "arg1".to_string(),
                        arg_type: "type1".to_string(),
                    },
                    WasmFnArgs {
                        value: serde_json::from_str("{}").unwrap(),
                        name: "arg2".to_string(),
                        arg_type: "type2".to_string(),
                    },
                ],
            },
        }
    }

    #[test]
    fn test_put_and_get_data() {
        let data = wasm_mocked();
        let mut rocksdb = RocksDB::dev();

        rocksdb.put("key01", data.clone()).unwrap();

        let result = rocksdb.get("key01").unwrap();
        assert_eq!(data, result);

        drop(rocksdb.db.lock().unwrap());
    }

    #[test]
    fn test_update_data() {
        let data = wasm_mocked();
        let mut rocksdb = RocksDB::dev();

        rocksdb.put("key02", data.clone()).unwrap();

        let mut updated_data = data.clone();
        updated_data.metadata.id = 456;
        rocksdb.put("key02", updated_data.clone()).unwrap();

        let result = rocksdb.get("key02").unwrap();
        assert_eq!(updated_data, result);

        drop(rocksdb.db.lock().unwrap());
    }

    #[test]
    fn test_delete_data() {
        let data = wasm_mocked();
        let mut rocksdb = RocksDB::dev();

        rocksdb.put("key03", data).unwrap();
        rocksdb.del("key03").unwrap();

        let result = rocksdb.get("key03");
        assert!(result.is_none());

        drop(rocksdb.db.lock().unwrap());
    }
}
