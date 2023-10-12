//! # The `engine` module provides a runtime environment for executing WebAssembly functions
//!
//! This module contains the following main components:
//!
//! - [`Runtime`]: A struct representing the runtime environment for WebAssembly functions.
//!
//! The `engine` module depends on the following modules:
//!
//! - [`CompiledWasmCache`]: A struct representing the cache for compiled WebAssembly modules.
//! - [`WasmModule`]: A struct representing a WebAssembly function.
//! - [`RunRequest`]: A struct representing a request to run a WebAssembly function.
//! - [`RunnerError`]: An enum representing the possible errors that can occur during the execution of a run job.

use crate::{
    database::models::WasmModule,
    logger,
    metrics::constants::{WASM_COMPILER_TIME, WASM_EXECUTION_TIME},
    workers::runner::models::RunnerError,
};
use std::time::Instant;
use wasmer::{imports, Instance, Module, Store, Value};

/// A runtime environment for executing WebAssembly functions.
pub struct Runtime {
    wasm_module: WasmModule,
    id: String,
}

impl Runtime {
    /// # Creates a new instance of the [`Runtime`] struct.
    ///
    /// ## Arguments
    ///
    /// * `wasm_module` - A [`WasmModule`] object that represents the WebAssembly function.
    pub fn new(wasm_module: WasmModule, id: String) -> Self {
        Self { wasm_module, id }
    }

    /// # Executes a WebAssembly function.
    ///
    /// ## Arguments
    ///
    /// * `wasm_args` - A [`RunRequest`] object that represents the function arguments.
    ///
    /// ## Returns
    ///
    /// * A [`Result<wasmer::Value, RunnerError>`] containing either the function's result or an error.
    pub fn run(&mut self, wasm_args: &[Value]) -> Result<Box<[wasmer::Value]>, RunnerError> {
        let start = Instant::now();
        let mut store = Store::default();
        let module = match Module::new(&store, self.wasm_module.wasm.clone()) {
            Ok(m) => m,
            Err(e) => {
                let e = logger::log_error(RunnerError::CompilingError(e.to_string()));
                return Err(e);
            }
        };
        let duration = start.elapsed().as_nanos() as f64;
        WASM_COMPILER_TIME
            .with_label_values(&[self.id.as_str()])
            .observe(duration);

        let import_object = imports! {};
        let instance = match Instance::new(&mut store, &module, &import_object) {
            Ok(i) => i,
            Err(e) => {
                let e = logger::log_error(RunnerError::InitializingError(e.to_string()));
                return Err(e);
            }
        };

        let wasm_function = match instance
            .exports
            .get_function(&self.wasm_module.metadata.function_name)
        {
            Ok(f) => f,
            Err(e) => {
                let e = logger::log_error(RunnerError::InstantiateFunctionError(
                    self.wasm_module.metadata.function_name.clone(),
                    e.to_string(),
                ));
                return Err(e);
            }
        };

        let start = Instant::now();
        let result = match wasm_function.call(&mut store, wasm_args) {
            Ok(r) => r,
            Err(e) => {
                let e = logger::log_error(RunnerError::FunctionExecutionError(e.to_string()));
                return Err(e);
            }
        };
        let duration = start.elapsed().as_nanos() as f64;
        WASM_EXECUTION_TIME
            .with_label_values(&[self.wasm_module.metadata.function_name.as_str()])
            .observe(duration);

        Ok(result)
    }
}
