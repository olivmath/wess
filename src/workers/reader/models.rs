use crate::database::models::WasmFn;
use serde::Serialize;
use tokio::sync::oneshot::Sender;

/// # Read Job Type
#[derive(Debug)]
pub struct RJob {
    pub responder: Sender<ReadResponse>,
    pub id: String,
}

impl RJob {
    pub fn new(responder: Sender<ReadResponse>, id: String) -> Self {
        Self { responder, id }
    }
}

/// # Read Response Type
#[derive(Serialize, Debug)]
pub enum ReadResponse {
    Success(WasmFn),
    Fail(String),
}

impl ReadResponse {
    /// # Success: [`WasmFn`]
    pub fn new(wasm_fn: WasmFn) -> Self {
        ReadResponse::Success(wasm_fn)
    }

    /// # Fail: [`String`]
    pub fn fail(msg: String) -> Self {
        ReadResponse::Fail(msg)
    }
}
