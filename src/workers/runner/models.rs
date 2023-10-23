//! # This `models` module provides types for the `runner` module
//!
//! This module contains the following types:
//!
//! - [`RunJob`]: A struct representing a run job, containing a [`RunRequest`], an ID, and a channel to send the [`RunResponse`].
//! - [`RunResponse`]: An enum representing the response to a run job. It can be either `Success`, containing a string with the output of the run job, or `Fail`, containing a [`RunnerError`].
//! - [`RunnerError`]: An enum representing the possible errors that can occur during the execution of a run job. It includes errors such as `InstantiateFunctionError`, `FunctionExecutionError`, `InitializingError`, `CompilingError`, and `WasmNotFound`.
//!
//! The `models` module depends on the following modules:
//!
//! - [`RunRequest`]: Represents a request to run a WebAssembly function.

use tokio::sync::oneshot::Sender;
use wasmer::Value;

use crate::errors::WessError;

/// Run Job Type
#[derive(Debug)]
pub struct RunJob {
    pub responder: Sender<RunResponse>,
    pub args: Vec<Value>,
    pub id: String,
}

impl RunJob {
    pub fn new(responder: Sender<RunResponse>, args: Vec<Value>, id: String) -> Self {
        Self {
            responder,
            args,
            id,
        }
    }
}

/// # Run Response Type
#[derive(Debug)]
pub enum RunResponse {
    Success(Box<[wasmer::Value]>),
    Fail(WessError),
}

impl RunResponse {
    /// # Success: [`String`]
    pub fn new(r: Box<[wasmer::Value]>) -> Self {
        RunResponse::Success(r)
    }

    /// # Fail: [`WessError`]
    pub fn fail(msg: WessError) -> Self {
        RunResponse::Fail(msg)
    }
}
