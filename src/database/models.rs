use serde::{Deserialize, Serialize};

/// Represents WebAssembly bytecode.
pub type Wasm = Vec<u8>;

/// # Represents an argument for a WebAssembly function.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct FnTypeArg {
    /// The name of the argument.
    pub name: String,
    /// The type of the argument.
    pub arg_type: String,
}

impl FnTypeArg {
    /// # Creates a new instance of the `FnTypeArg` structure.
    ///
    /// ## Arguments
    ///
    /// * `name` - A string slice that represents the name of the argument.
    /// * `arg_type` - A string slice that represents the type of the argument.
    ///
    /// ## Returns
    ///
    /// * An instance of `FnTypeArg`.
    pub fn new(name: String, arg_type: String) -> Self {
        Self { name, arg_type }
    }
}

/// # Represents metadata associated with a WebAssembly function.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct WasmMetadata {
    // pub owner: Vec<u8>,
    // pub signature: Vec<u8>,
    /// The name of the function.
    pub func: String,
    /// The return type of the function.
    pub func_return_type: String,
    /// A vector of function argument types.
    pub args: Vec<Option<FnTypeArg>>,
}

impl WasmMetadata {
    /// # Creates a new instance of the `WasmMetadata` structure.
    ///
    /// ## Arguments
    ///
    /// * `func` - A string slice that represents the name of the function.
    /// * `func_return_type` - A string slice that represents the return type of the function.
    /// * `args` - A vector of `Option<FnTypeArg>` objects that represent the function argument types.
    ///
    /// ## Returns
    ///
    /// * An instance of `WasmMetadata`.
    pub fn new(func: String, func_return_type: String, args: Vec<Option<FnTypeArg>>) -> Self {
        Self {
            func,
            func_return_type,
            args,
        }
    }
}

/// # Represents a WebAssembly function.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct WasmFn {
    /// The WebAssembly bytecode.
    pub wasm: Wasm,
    /// The metadata associated with the function.
    pub metadata: WasmMetadata,
}

impl WasmFn {
    /// # Creates a new instance of the `WasmFn` structure.
    ///
    /// ## Arguments
    ///
    /// * `wasm` - A `Wasm` object that represents the WebAssembly bytecode.
    /// * `metadata` - A `WasmMetadata` object that represents the metadata associated with the function.
    ///
    /// ## Returns
    ///
    /// * An instance of `WasmFn`.
    pub fn new(wasm: Wasm, metadata: WasmMetadata) -> Self {
        Self { wasm, metadata }
    }
}
