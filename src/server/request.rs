use super::super::wasm::WasmFnArgs;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum JobType {
    Modity,
    Delete,
    Save,
    Run,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WasmRequest {
    pub wasm: Vec<u8>,
    pub run_function: String,
    pub args: Vec<Option<WasmFnArgs>>,
}

#[derive(Debug, Clone)]
pub struct WasmJob {
    pub id: String,
    pub wasm_req: WasmRequest,
    pub job_type: JobType,
}
