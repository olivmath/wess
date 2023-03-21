use std::sync::{Arc, Mutex};

use rocksdb::{DBWithThreadMode, Error, MultiThreaded, Options, DB as DataBase};

use lazy_static::lazy_static;

lazy_static! {
    static ref DB: Arc<Mutex<DBWithThreadMode<MultiThreaded>>> = {
        let path = "./rocksdb";
        let mut options = Options::default();
        options.create_if_missing(true);

        match DataBase::open_default(path) {
            Ok(db) => Arc::new(Mutex::new(db)),
            Err(err) => panic!("DB dont open: {}", err),
        }
    };
}

struct RocksDB {
    db: Arc<Mutex<DBWithThreadMode<MultiThreaded>>>,
}

impl RocksDB {
    fn new() -> Self {
        RocksDB {
            db: Arc::clone(&DB),
        }
    }

    pub fn put(&mut self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        self.db.lock().unwrap().put(key.as_bytes(), &value)
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.db.lock().unwrap().get(key.as_bytes()).unwrap()
    }

    pub fn del(&mut self, key: &str) -> Result<(), Error> {
        self.db.lock().unwrap().delete(key.as_bytes())
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
