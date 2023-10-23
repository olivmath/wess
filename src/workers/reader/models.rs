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

use crate::{database::models::WasmModule, errors::WessError};
use serde::Serialize;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub struct ReadJob {
    pub tx: Sender<ReadResponse>,
    pub id: Option<String>,
}

impl ReadJob {
    pub fn new(tx: Sender<ReadResponse>, id: Option<String>) -> Self {
        Self { tx, id }
    }
}

#[derive(Serialize, Debug)]
pub enum ReadResponse {
    Module(WasmModule),
    Fail(WessError),
    Size(usize),
}
