use super::super::database::RocksDB;
use super::super::wasm::Wasm;
use serde_json::json;
use tide::{Error, Response};

pub fn all_wasm() -> Result<Response, Error> {
    let db = RocksDB::new();

    let wasms: Vec<Wasm> = db.get_all().into_iter().filter_map(|v| v).collect();
    Ok(Response::builder(200).body(json!(wasms)).build())
}
