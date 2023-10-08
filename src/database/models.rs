//! # This `models` module provides types for `server` module
//!
//! This module contains the following types:
//!
//! - [`Wasm`]: A type alias for a [`Vec<u8>`] representing WebAssembly bytecode.
//! - [`TypeArg`]: A struct representing an argument for a WebAssembly function, containing a name and a type.
//! - [`WasmMetadata`]: A struct representing metadata associated with a WebAssembly function, containing its name, return type and a vector of function argument types.
//! - [`WasmModule`]: A struct representing a WebAssembly function, containing its bytecode and metadata.
//!
//! All types are serializable and deserializable through serde.

use serde::{Deserialize, Serialize};

/// # Represents WebAssembly bytecode.
pub type Wasm = Vec<u8>;

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub enum TypeArg {
    #[default]
    I32,
    I64,
    F32,
    F64,
}

/// # Represents metadata associated with a WebAssembly function.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct WasmMetadata {
    // pub owner: Vec<u8>,
    // pub signature: Vec<u8>,
    /// The name of the function.
    #[serde(rename = "functionName")]
    pub function_name: String,
    /// The return type of the function.
    #[serde(rename = "returnType")]
    pub return_type: TypeArg,
    /// A vector of function argument types.
    pub args: Vec<Option<TypeArg>>,
}

/// # Represents a WebAssembly function.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct WasmModule {
    /// The WebAssembly bytecode.
    pub wasm: Wasm,
    /// The metadata associated with the function.
    pub metadata: WasmMetadata,
}

impl WasmMetadata {
    /// # Creates a new instance of the [`WasmMetadata`] structure.
    ///
    /// ## Arguments
    ///
    /// * `func` - A string slice that represents the name of the function.
    /// * `return_type` - A string slice that represents the return type of the function.
    /// * `args` - A vector of [`Option<TypeArg>`] objects that represent the function argument types.
    ///
    /// ## Returns
    ///
    /// * An instance of [`WasmMetadata`].
    pub fn new(function_name: String, return_type: TypeArg, args: Vec<Option<TypeArg>>) -> Self {
        Self {
            function_name,
            return_type,
            args,
        }
    }
}

impl WasmModule {
    /// # Creates a new instance of the [`WasmModule`] structure.
    ///
    /// ## Arguments
    ///
    /// * `wasm` - A [`Wasm`] object that represents the WebAssembly bytecode.
    /// * `metadata` - A [`WasmMetadata`] object that represents the metadata associated with the function.
    ///
    /// ## Returns
    ///
    /// * An instance of [`WasmModule`].
    pub fn new(wasm: Wasm, metadata: WasmMetadata) -> Self {
        Self { wasm, metadata }
    }
    /// # Convert the [`Wasm`] bytecode of a [`WasmModule`] instance to a byte slice.
    ///
    /// This method returns the WebAssembly bytecode of the [`WasmModule`] instance as a byte slice.
    ///
    /// ## Returns
    ///
    /// * A byte slice `&[u8]` representing the WebAssembly bytecode of the [`WasmModule`] instance.
    pub fn to_binary(&self) -> &[u8] {
        self.wasm.as_slice()
    }
}
