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
    errors::WessError,
    metrics::constants::{WASM_COMPILER_TIME, WASM_EXECUTION_TIME},
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
    /// * A [`Result<wasmer::Value, WessError>`] containing either the function's result or an error.
    pub fn run(&mut self, wasm_args: &[Value]) -> Result<Box<[wasmer::Value]>, WessError> {
        let start = Instant::now();
        // TODO: make statefull
        // save the store in DB
        let mut store = Store::default();
        let module = unsafe {
            match Module::from_binary_unchecked(&store, &self.wasm_module.wasm) {
                Ok(m) => m,
                Err(e) => {
                    let werr = log_error!(format!("Compiling Error: {}", e.to_string()), 500);
                    return Err(werr);
                }
            }
        };
        let duration = start.elapsed();
        WASM_COMPILER_TIME
            .with_label_values(&[self.id.as_str()])
            .observe(duration.as_secs_f64());

        let import_object = imports! {};
        let instance = match Instance::new(&mut store, &module, &import_object) {
            Ok(i) => i,
            Err(e) => {
                let werr = log_error!(format!("InitializingError: {}", e.to_string()), 500);
                return Err(werr);
            }
        };

        let wasm_function = match instance
            .exports
            .get_function(&self.wasm_module.metadata.function_name)
        {
            Ok(f) => f,
            Err(e) => {
                let werr = log_error!(
                    format!(
                        "Instantiate Function Error `{}`: {}",
                        self.wasm_module.metadata.function_name.clone(),
                        e.to_string()
                    ),
                    500
                );
                return Err(werr);
            }
        };

        let start = Instant::now();
        let result = match wasm_function.call(&mut store, wasm_args) {
            Ok(r) => r,
            Err(e) => {
                let werr = log_error!(format!("Function Execution Error: {}", e.to_string()), 500);
                return Err(werr);
            }
        };
        let duration = start.elapsed();
        WASM_EXECUTION_TIME
            .with_label_values(&[self.wasm_module.metadata.function_name.as_str()])
            .observe(duration.as_secs_f64());

        Ok(result)
    }
}
