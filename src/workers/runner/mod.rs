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
    engine::{cache::CompiledWasmCache, Runtime},
    models::{RunJob, RunResponse, RunnerError},
};
use super::reader::cache::Cache;
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
use wasmtime::Engine;

/// An async executor for running WebAssembly functions.
pub struct Runner {
    compiled_wasm_cache: Arc<Mutex<CompiledWasmCache>>,
    rx: Receiver<RunJob>,
    engine: Engine,
    cache: Cache,
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
        let compiled_wasm_cache = Arc::new(Mutex::new(CompiledWasmCache::new()));
        let channel_size = CONFIG.runner.channel_size;
        let (tx, rx) = mpsc::channel::<RunJob>(channel_size);
        let engine = Engine::default();
        let cache = Cache::new();
        (
            tx,
            Arc::new(Mutex::new(Runner {
                compiled_wasm_cache,
                engine,
                cache,
                rx,
                db,
            })),
        )
    }

    /// # Starts the [`Runner`].
    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let mut compiled_wasm_cache = self.compiled_wasm_cache.lock().await;
            let responder = job.responder;
            let engine = &self.engine;
            let args = job.args;
            let db = &self.db;
            let id = job.id;
            //
            match self.cache.get(id.clone(), || db.get(id.as_str())) {
                Some(wasm_fn) => match self
                    .run_function(&mut *compiled_wasm_cache, args, wasm_fn, &engine, id)
                    .await
                {
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
        cache: &mut CompiledWasmCache,
        args: RunRequest,
        wasm_fn: WasmFn,
        engine: &Engine,
        id: String,
    ) -> Result<RunResponse, RunnerError> {
        let mut runtime = Runtime::new(wasm_fn.clone());
        match runtime.run(cache, args, engine, id) {
            Ok(r) => Ok(RunResponse::new(r.to_string())),
            Err(e) => Err(e),
        }
    }
}
