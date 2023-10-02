//! # The [`Runner`] module provides an async executor for running WebAssembly functions
//!
//! This module contains the following main components:
//!
//! - [`Runner`]: A struct representing the async executor.
//!
//! The `runner` module depends on the following modules:
//!
//! - [`models`]: A module that contains the models for wrapping data sent over channels.
//!
//! The [`Runner`] is responsible for receiving and executing WebAssembly functions through channels,
//! managing the compiled WebAssembly cache, and interacting with the database.
//!
//! This module is responsible for running WebAssembly functions in an asynchronous and efficient manner.

pub mod engine;
pub mod models;

use self::{
    engine::Runtime,
    models::{RunJob, RunResponse, RunnerError},
};
use crate::{
    config::CONFIG,
    database::{models::WasmFn, RocksDB},
    server::models::RunRequest,
};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

/// An async executor for running WebAssembly functions.
pub struct Runner {
    rx: Receiver<RunJob>,
    db: RocksDB,
}

impl Runner {
    /// # Creates a new instance of the [`Runner`] struct.
    ///
    /// ## Arguments
    ///
    /// * `db` - A [`RocksDB`] object that represents the database.
    ///
    /// ## Returns
    ///
    /// * A tuple containing a [`Sender<RunJob>`] and an [`Arc<Mutex<Runner>>`].
    pub fn new(db: RocksDB) -> (Sender<RunJob>, Arc<Mutex<Runner>>) {
        let channel_size = CONFIG.runner.channel_size;
        let (tx, rx) = mpsc::channel::<RunJob>(channel_size);
        (tx, Arc::new(Mutex::new(Runner { rx, db })))
    }

    /// # Starts the [`Runner`].
    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let responder = job.responder;
            let args = job.args;
            let db = &self.db;
            let id = job.id;
            //
            match db.get(id.as_str()) {
                Some(wasm_fn) => match self.run_function(args, wasm_fn).await {
                    Ok(result) => {
                        tokio::spawn(async move { responder.send(result) });
                    }
                    Err(e) => {
                        tokio::spawn(async move { responder.send(RunResponse::Fail(e)) });
                    }
                },
                None => {
                    tokio::spawn(async move {
                        responder.send(RunResponse::fail(RunnerError::WasmNotFound))
                    });
                }
            };
        }
    }

    /// # Runs a WebAssembly function.
    ///
    /// ## Arguments
    ///
    /// * `cache` - A mutable reference to a [`CompiledWasmCache`] for caching the compiled WebAssembly modules.
    /// * `args` - A [`RunRequest`] object that represents the function arguments.
    /// * `wasm_fn` - A [`WasmFn`] object that represents the WebAssembly function.
    /// * `engine` - A reference to a [`wasmtime::Engine`] object for running the WebAssembly module.
    /// * `id` - A [`String`] representing the ID of the WebAssembly function.
    ///
    /// ## Returns
    ///
    /// * A [`Result<RunResponse, RunnerError>`] containing either the function's result or an error.
    pub async fn run_function(
        &self,
        args: RunRequest,
        wasm_fn: WasmFn,
    ) -> Result<RunResponse, RunnerError> {
        let mut runtime = Runtime::new(wasm_fn.clone());
        match runtime.run(args) {
            Ok(r) => Ok(RunResponse::new(r)),
            Err(e) => Err(e),
        }
    }
}
