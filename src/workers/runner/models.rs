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

use serde::Serialize;
use std::fmt;
use tokio::sync::oneshot::Sender;
use wasmer::Value;

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
    Fail(RunnerError),
}

impl RunResponse {
    /// # Success: [`String`]
    pub fn new(r: Box<[wasmer::Value]>) -> Self {
        RunResponse::Success(r)
    }

    /// # Fail: [`RunnerError`]
    pub fn fail(msg: RunnerError) -> Self {
        RunResponse::Fail(msg)
    }
}

/// # RunnerError Enum
///
/// Enum representing the possible errors that can occur during the execution of a run job.
///
/// Variants:
///
/// - `InstantiateFunctionError(String, String)`: An error occurred while trying to instantiate the WebAssembly function. The first `String` is the name of the function, and the second `String` is the error message.
/// - `FunctionExecutionError(String)`: An error occurred during the execution of the WebAssembly function. The `String` contains details about the error.
/// - `InitializingError(String)`: An error occurred while initializing the runner. The `String` contains details about the error.
/// - `CompilingError(String)`: An error occurred while compiling the WebAssembly module. The `String` contains details about the error.
/// - `WasmNotFound`: The WebAssembly module was not found.
#[derive(Serialize, Debug)]
pub enum RunnerError {
    InstantiateFunctionError(String, String),
    FunctionExecutionError(String),
    InitializingError(String),
    CompilingError(String),
    WasmNotFound,
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunnerError::InstantiateFunctionError(name, err) => {
                write!(f, "Error instantiating function {}: {}", name, err)
            }
            RunnerError::FunctionExecutionError(err) => {
                write!(f, "Error executing function: {}", err)
            }
            RunnerError::InitializingError(err) => {
                write!(f, "Error initializing runner: {}", err)
            }
            RunnerError::CompilingError(err) => {
                write!(f, "Error compiling wasm module: {}", err)
            }
            RunnerError::WasmNotFound => write!(f, "Wasm module not found"),
        }
    }
}
