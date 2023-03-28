//! # This `models` module provides types for `runner` module
//!
//! This module contains the following types:
//!
//! - [`RunJob`]: A struct representing a run job, containing a [`RunRequest`], an ID, and a channel to send the [`RunResponse`].
//! - [`RunResponse`]: An enum representing the response to a run job. It can be either [`Success`], containing a string with the output of the run job, or [`Fail`], containing a [`RunnerError`].
//! - [`RunnerError`]: An enum representing the possible errors that can occur during the execution of a run job. It can be either a `WasmNotFound` error, an `Execution` error with a string containing details about the error, or an `Unknown` error with a string describing the error.
//!
//! The `models` module depends on the following modules:
//!
//! - [`RunRequest`]: Represents a request to run a WebAssembly function.

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
