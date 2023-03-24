use crate::server::request::WasmRequest;
use serde;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::Response;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WasmFnArgs {
    pub value: serde_json::Value,
    pub name: String,
    #[serde(rename = "type")]
    pub arg_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct WasmMetadata {
    // pub owner: Vec<u8>,
    // pub signature: Vec<u8>,
    pub id: String,
    pub fn_main: String,
    pub args: Vec<Option<WasmFnArgs>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Wasm {
    pub wasm: Vec<u8>,
    pub metadata: WasmMetadata,
}

impl Into<Response> for Wasm {
    fn into(self) -> Response {
        Response::builder(200).body(json!(self.wasm)).build()
    }
}

impl Into<Vec<u8>> for Wasm {
    fn into(self) -> Vec<u8> {
        self.wasm
    }
}

#[derive(Debug, Clone)]
pub enum JobType {
    Modity,
    Delete,
    Save,
    Run,
}

#[derive(Debug, Clone)]
pub struct WasmJob {
    pub id: String,
    pub wasm_req: WasmRequest,
    pub job_type: JobType,
}

impl WasmJob {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn to_wasm(self) -> Wasm {
        Wasm {
            wasm: self.wasm_req.wasm,
            metadata: WasmMetadata {
                id: self.id,
                fn_main: self.wasm_req.run_function,
                args: self.wasm_req.args,
            },
        }
    }
}
