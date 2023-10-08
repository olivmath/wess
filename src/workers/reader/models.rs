//! # This `models` module provides types for `reader` module
//!
//! This module contains the following types:
//!
//! - [`ReadJob`]: A struct representing a read job, containing a channel to send the read response and the ID of the wasm function to be read.
//! - [`ReadResponse`]: An enum representing the response of a read operation. It can either contain the retrieved wasm function, or a message indicating that the function was not found.
//!
//! The `models` module depends on the following modules:
//!
//! - [`WasmModule`]: Represents a WebAssembly function.

use crate::database::models::WasmModule;
use serde::Serialize;
use tokio::sync::oneshot::Sender;

/// # Read Job Type
#[derive(Debug)]
pub struct ReadJob {
    pub responder: Sender<ReadResponse>,
    pub id: String,
}

impl ReadJob {
    /// # Creates a new read job
    ///
    /// ## Arguments
    ///
    /// * `responder` - Channel to send the read response
    /// * `id` - ID of the wasm function to be read
    pub fn new(responder: Sender<ReadResponse>, id: String) -> Self {
        Self { responder, id }
    }
}

/// # Read Response Type
#[derive(Serialize, Debug)]
pub enum ReadResponse {
    Success(WasmModule),
    Fail(String),
}

impl ReadResponse {
    /// # Creates a success response with the retrieved [`WasmModule`]
    ///
    /// ## Arguments
    ///
    /// * `wasm_module` - Retrieved wasm function
    pub fn new(wasm_module: WasmModule) -> Self {
        ReadResponse::Success(wasm_module)
    }

    /// # Creates a fail response with a message
    ///
    /// ## Arguments
    ///
    /// * `msg` - Error message
    pub fn fail(msg: String) -> Self {
        ReadResponse::Fail(msg)
    }
}
