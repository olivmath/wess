//! # This `models` module provides types for `reader` module
//!
//! This module contains the following types:
//!
//! - [`RJob`]: A struct representing a read job, containing a channel to send the read response and the ID of the wasm function to be read.
//! - [`ReadResponse`]: An enum representing the response of a read operation. It can either contain the retrieved wasm function, or a message indicating that the function was not found.
//!
//! The `models` module depends on the following modules:
//!
//! - [`WasmFn`]: Represents a WebAssembly function.

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
    Success(WasmFn),
    Fail(String),
}

impl ReadResponse {
    /// # Creates a success response with the retrieved [`WasmFn`]
    ///
    /// ## Arguments
    ///
    /// * `wasm_fn` - Retrieved wasm function
    pub fn new(wasm_fn: WasmFn) -> Self {
        ReadResponse::Success(wasm_fn)
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
