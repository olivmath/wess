use crate::wasm::WasmJob;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct Job {
    pub wasm_job: WasmJob,
    pub responder: Sender<String>,
}
