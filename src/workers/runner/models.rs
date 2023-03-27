use crate::server::models::RunRequest;
use serde::Serialize;
use tokio::sync::oneshot::Sender;

/// Run Job Type
#[derive(Debug)]
pub struct RunJob {
    pub responder: Sender<RunResponse>,
    pub args: RunRequest,
    pub id: String,
}

impl RunJob {
    pub fn new(responder: Sender<RunResponse>, args: RunRequest, id: String) -> Self {
        Self {
            responder,
            args,
            id,
        }
    }
}

/// # Run Response Type
#[derive(Debug, Serialize)]
pub enum RunResponse {
    Success(String),
    Fail(RunnerError),
}

impl RunResponse {
    /// # Success: [`String`]
    pub fn new(r: String) -> Self {
        RunResponse::Success(r)
    }

    /// # Fail: [`RunnerError`]
    pub fn fail(msg: RunnerError) -> Self {
        RunResponse::Fail(msg)
    }
}

#[derive(Serialize, Debug)]
pub enum RunnerError {
    Execution(String),
    Unknown(String),
    WasmNotFound,
}
