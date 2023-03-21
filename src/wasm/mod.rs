pub mod signature;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WasmFnArgs {
    pub value: serde_json::Value,
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WasmMetadata {
    pub owner: Vec<u8>,
    pub signature: Vec<u8>,
    pub id: u32,
    pub fn_main: String,
    pub args: Vec<WasmFnArgs>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Wasm {
    pub wasm: Vec<u8>,
    pub metadata: WasmMetadata,
}
