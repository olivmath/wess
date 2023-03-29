//! # The `engine` module provides a runtime environment for executing WebAssembly functions
//!
//! This module contains the following main components:
//!
//! - [`Runtime`]: A struct representing the runtime environment for WebAssembly functions.
//! - [`cache`]: A submodule providing a cache for compiled WebAssembly modules.
//!
//! The `engine` module depends on the following modules:
//!
//! - [`CompiledWasmCache`]: A struct representing the cache for compiled WebAssembly modules.
//! - [`WasmFn`]: A struct representing a WebAssembly function.
//! - [`RunRequest`]: A struct representing a request to run a WebAssembly function.
//! - [`RunnerError`]: An enum representing the possible errors that can occur during the execution of a run job.

pub mod cache;

use self::cache::CompiledWasmCache;
use crate::{
    database::models::WasmFn, server::models::RunRequest, workers::runner::models::RunnerError,
};
use wasmtime::{Engine, Linker, Store};

/// A runtime environment for executing WebAssembly functions.
pub struct Runtime {
    wasm_fn: WasmFn,
}

impl Runtime {
    /// # Creates a new instance of the [`Runtime`] struct.
    ///
    /// ## Arguments
    ///
    /// * `wasm_fn` - A [`WasmFn`] object that represents the WebAssembly function.
    pub fn new(wasm_fn: WasmFn) -> Self {
        Self { wasm_fn }
    }

    /// # Executes a WebAssembly function.
    ///
    /// ## Arguments
    ///
    /// * `cache` - A mutable reference to a [`CompiledWasmCache`] for caching the compiled WebAssembly modules.
    /// * `_args` - A [`RunRequest`] object that represents the function arguments.
    /// * `engine` - A reference to a [`wasmtime::Engine`] object for running the WebAssembly module.
    /// * `id` - A `String` representing the ID of the WebAssembly function.
    ///
    /// ## Returns
    ///
    /// * A [`Result<i32, RunnerError>`] containing either the function's result or an error.
    pub fn run(
        &mut self,
        cache: &mut CompiledWasmCache,
        _args: RunRequest,
        engine: &Engine,
        id: String,
    ) -> Result<i32, RunnerError> {
        let wasm_file = self.wasm_fn.to_binary();
        let function_name = &self.wasm_fn.metadata.func;

        let module = match cache.get(id, engine.clone(), || wasm_file.to_vec()) {
            Ok(m) => m,
            Err(err) => {
                let e = RunnerError::CompilingError(err.to_string());
                eprintln!("{e}");
                return Err(e);
            }
        };

        let linker = Linker::new(&engine);
        let mut storage = Store::new(&engine, ());
        let instance = match linker.instantiate(&mut storage, &module) {
            Ok(i) => i,
            Err(err) => {
                let e = RunnerError::InitializingError(err.to_string());
                eprintln!("{e}");
                return Err(e);
            }
        };

        let function = match instance.get_typed_func::<(), i32>(&mut storage, &function_name) {
            Ok(f) => f,
            Err(err) => {
                let e = RunnerError::InstantiateFunctionError(
                    function_name.to_string(),
                    err.to_string(),
                );
                eprintln!("{e}");
                return Err(e);
            }
        };

        let result = match function.call(storage, ()) {
            Ok(r) => r,
            Err(err) => {
                let e = RunnerError::FunctionExecutionError(err.to_string());
                eprintln!("{e}");
                return Err(e);
            }
        };

        Ok(result)
    }
}
