use lazy_static::lazy_static;
use rocksdb::{DBWithThreadMode, Error, MultiThreaded, Options, DB as DataBase};
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

    /// Stores a value in the database associated with the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that represents the key.
    /// * `value` - An array of bytes representing the value to be stored.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - An empty result on success or an error on failure.
    pub fn put(&mut self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        self.db.lock().unwrap().put(key.as_bytes(), &value)
    }

    /// Retrieves a value from the database based on the supplied key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string that represents the key.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<u8>>` - An option containing a byte array if the key exists or `None` if the key does not exist.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.db.lock().unwrap().get(key.as_bytes()).unwrap()
    }

    /// Deletes a value from the database based on the given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string representing the key.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - An empty result on success or an error on failure.
    pub fn del(&mut self, key: &str) -> Result<(), Error> {
        self.db.lock().unwrap().delete(key.as_bytes())
    }

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
    use crate::wasm::wasm::{Wasm, WasmFnArgs, WasmMetadata};

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
        let mut db = RocksDB::new();

        let key = "mykey";
        let value = serde_json::to_vec(&data).unwrap();

        db.put(key, value).unwrap();
        let result = {
            let result = db.get(key).unwrap();
            serde_json::from_slice(&result).unwrap()
        };

        assert_eq!(data, result);
    }

    #[test]
    fn test_update_data() {
        let data = wasm_mocked();
        let mut db = RocksDB::new();

        let key = "update_key";
        let value = serde_json::to_vec(&data).unwrap();
        db.put(key, value).unwrap();

        let mut updated_data = data.clone();
        let updated_value = {
            updated_data.metadata.id = 456;
            serde_json::to_vec(&updated_data).unwrap()
        };
        db.put(key, updated_value).unwrap();

        let result = {
            let result = db.get(key).unwrap();
            serde_json::from_slice(&result).unwrap()
        };
        assert_eq!(updated_data, result);
    }

    #[test]
    fn test_delete_data() {
        let data = wasm_mocked();
        let mut db = RocksDB::new();

        let key = "delete_key";
        let value = serde_json::to_vec(&data).unwrap();

        db.put(key, value).unwrap();
        db.del(key).unwrap();

        let result = db.get(key);
        assert!(result.is_none());
    }
}
