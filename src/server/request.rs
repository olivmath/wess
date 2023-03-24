use super::super::wasm::WasmFnArgs;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct WasmRequest {
    pub wasm: Vec<u8>,
    pub run_function: String,
    pub args: Vec<Option<WasmFnArgs>>,
}
