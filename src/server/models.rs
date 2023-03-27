//! # The models module provides types and functions for handling incoming HTTP requests to the Wess application.
//!
//! This module contains the following main components:
//!
//! - [`FnTypeArgRequest`]: A struct that represents the type of an argument in a WebAssembly function request.
//! - [`WasmRequest`]: A struct representing a request to execute a WebAssembly function.
//! - [`WRequest`]: A struct representing a write request type.
//! - [`RequestError`]: An enumeration of potential errors that can occur when parsing a request.
//! - [`FnArg`]: A struct representing an argument for a WebAssembly function.
//! - [`RunRequest`]: A struct representing a request to run a WebAssembly function.

use crate::database::models::{FnTypeArg, Wasm, WasmFn, WasmMetadata};
use serde;
use serde::Deserialize;

/// Represents the type of an argument in a WebAssembly function request.
#[derive(Deserialize, Clone, Debug)]
pub struct FnTypeArgRequest {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub name: String,
}

impl From<FnTypeArgRequest> for FnTypeArg {
    fn from(request: FnTypeArgRequest) -> Self {
        FnTypeArg::new(request.name, request.arg_type)
    }
}

/// Represents a request to execute a WebAssembly function.
#[derive(Deserialize, Clone, Default, Debug)]
pub struct WasmRequest {
    /// The arguments for the WebAssembly function.
    pub args: Vec<Option<FnTypeArgRequest>>,
    /// The return type of the WebAssembly function.
    pub return_type: String,
    /// The WebAssembly bytecode.
    pub wasm: Wasm,
    /// The name of the WebAssembly function to execute.
    pub func: String,
}

/// Represents a write request type.
#[derive(Deserialize, Clone)]
pub struct WRequest(pub Option<WasmRequest>);

/// Represents an error that can occur when parsing a request.
#[derive(Debug)]
pub enum RequestError {
    /// Indicates that the JSON in the request is invalid.
    InvalidJson,
}

impl WRequest {
    /// # Converts a `WRequest` object to a `WasmFn` object.
    ///
    /// ## Returns
    ///
    /// * A `Result` containing the `WasmFn` object or
    /// a `RequestError` if there was an error parsing the request.
    pub fn to_wasm_fn(&self) -> Result<WasmFn, RequestError> {
        if let Some(wasm_req) = self.0.clone() {
            let wasm_metadata = WasmMetadata::new(
                wasm_req.func,
                wasm_req.return_type,
                wasm_req
                    .args
                    .into_iter()
                    .map(|item| item.map(FnTypeArg::from))
                    .collect(),
            );
            let wasm_fn = WasmFn::new(wasm_req.wasm, wasm_metadata);
            Ok(wasm_fn)
        } else {
            Err(RequestError::InvalidJson)
        }
    }
}

/// Represents an argument for a WebAssembly function.
#[derive(Deserialize, Debug)]
pub struct FnArg {
    /// The value of the argument.
    pub value: serde_json::Value,
    /// The name of the argument.
    pub name: String,
}

/// Represents a request to run a WebAssembly function.
#[derive(Deserialize, Debug)]
pub struct RunRequest {
    /// The arguments for the WebAssembly function.
    pub args: Vec<Option<FnArg>>,
}
