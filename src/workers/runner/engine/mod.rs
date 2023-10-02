//! # The `engine` module provides a runtime environment for executing WebAssembly functions
//!
//! This module contains the following main components:
//!
//! - [`Runtime`]: A struct representing the runtime environment for WebAssembly functions.
//!
//! The `engine` module depends on the following modules:
//!
//! - [`CompiledWasmCache`]: A struct representing the cache for compiled WebAssembly modules.
//! - [`WasmFn`]: A struct representing a WebAssembly function.
//! - [`RunRequest`]: A struct representing a request to run a WebAssembly function.
//! - [`RunnerError`]: An enum representing the possible errors that can occur during the execution of a run job.

use crate::{
    database::models::WasmFn, logger, server::models::RunRequest,
    workers::runner::models::RunnerError,
};

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
    /// * `wasm_args` - A [`RunRequest`] object that represents the function arguments.
    ///
    /// ## Returns
    ///
    /// * A [`Result<serde_json::Value, RunnerError>`] containing either the function's result or an error.
    pub fn run(&mut self, wasm_args: RunRequest) -> Result<serde_json::Value, RunnerError> {
        use wasmer::{imports, Instance, Module, Store, Value};

        // TODO: compile time metrics
        let mut store = Store::default();
        let module = match Module::new(&store, self.wasm_fn.wasm.clone()) {
            Ok(m) => m,
            Err(e) => {
                let e = logger::log_error(RunnerError::CompilingError(e.to_string()));
                return Err(e);
            }
        };

        let import_object = imports! {};
        let instance = match Instance::new(&mut store, &module, &import_object) {
            Ok(i) => i,
            Err(e) => {
                let e = logger::log_error(RunnerError::InitializingError(e.to_string()));
                return Err(e);
            }
        };

        let wasm_function = match instance.exports.get_function(&self.wasm_fn.metadata.func) {
            Ok(f) => f,
            Err(e) => {
                let e = logger::log_error(RunnerError::InstantiateFunctionError(
                    self.wasm_fn.metadata.func.clone(),
                    e.to_string(),
                ));
                return Err(e);
            }
        };

        let arg_types = &self.wasm_fn.metadata.args;
        let arg_values = &wasm_args.args;
        let mut dynamic_args = Vec::new();
        for (arg_type, arg_value) in arg_types.iter().zip(arg_values.iter()) {
            if let (Some(t), Some(v)) = (arg_type, arg_value) {
                let x = match t.arg_type.as_str() {
                    "i64" => {
                        if let Some(r) = v.value.as_i64() {
                            Value::I64(r)
                        } else {
                            return Err(RunnerError::TypeMismatchError(
                                "i64".to_string(),
                                get_value_type(&v.value),
                            ));
                        }
                    }
                    "i32" => {
                        if let Some(r) = v.value.as_i64() {
                            Value::I32(r as i32)
                        } else {
                            return Err(RunnerError::TypeMismatchError(
                                "i32".to_string(),
                                get_value_type(&v.value),
                            ));
                        }
                    }
                    "f64" => {
                        if let Some(r) = v.value.as_f64() {
                            Value::F64(r)
                        } else {
                            return Err(RunnerError::TypeMismatchError(
                                "f64".to_string(),
                                get_value_type(&v.value),
                            ));
                        }
                    }
                    "f32" => {
                        if let Some(r) = v.value.as_f64() {
                            Value::F32(r as f32)
                        } else {
                            return Err(RunnerError::TypeMismatchError(
                                "f32".to_string(),
                                get_value_type(&v.value),
                            ));
                        }
                    }
                    _ => return Err(RunnerError::ArgsError),
                };
                dynamic_args.push(x);
            } else {
                continue;
            }
        }

        let result = match wasm_function.call(&mut store, &dynamic_args) {
            Ok(r) => {
                let value = r.first().unwrap().clone();
                match self.wasm_fn.metadata.return_type.as_str() {
                    "i32" => serde_json::to_value(value.i32()),
                    "i64" => serde_json::to_value(value.i64()),
                    "f32" => serde_json::to_value(value.f32()),
                    "f64" => serde_json::to_value(value.f64()),
                    _ => return Err(RunnerError::ArgsError),
                }
            }
            Err(e) => {
                let e = logger::log_error(RunnerError::FunctionExecutionError(e.to_string()));
                return Err(e);
            }
        };

        Ok(result.unwrap())
    }
}

fn get_value_type(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Number(n) => {
            if n.is_f64() {
                "f64".to_string()
            } else if n.is_i64() {
                "i32".to_string()
            } else {
                "number".to_string()
            }
        },
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(_) => "boolean".to_string(),
        serde_json::Value::String(_) => "string".to_string(),
        serde_json::Value::Array(_) => "array".to_string(),
        serde_json::Value::Object(_) => "object".to_string(),
    }
}
