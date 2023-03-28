//! # The `Runner` module provides a async executor for write data into database
//!
//! This module contains the following main components:
//!
//! - [`Runner`]: A struct representing the async executor.
//!
//! The `runner` module depends on the following modules:
//!
//! - [`models`]: A module that contains the models for wrap data by channels.

pub mod engine;
pub mod models;

use self::{models::{RunJob, RunResponse, RunnerError}, engine::Runtime};
use crate::{
    database::{models::WasmFn, RocksDB},
    server::models::RunRequest,
};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

/// An async executor for running WebAssembly function.
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
        let (tx, rx) = mpsc::channel::<RunJob>(100);
        (tx, Arc::new(Mutex::new(Runner { rx, db })))
    }

    /// # Starts the [`Runner`].
    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let responder = job.responder;
            let args = job.args;
            let id = job.id;
            //
            match self.db.get(id.as_str()) {
                Some(wasm_fn) => match self.run_function(wasm_fn, args).await {
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
    /// * `wasm_fn` - A [`WasmFn`] object that represents the WebAssembly function.
    /// * `args` - A [`RunRequest`] object that represents the function arguments.
    ///
    /// ## Returns
    ///
    /// * A [`Result<RunResponse, RunnerError>`] containing either the function's result or an error.
    pub async fn run_function(
        &self,
        wasm_fn: WasmFn,
        args: RunRequest,
    ) -> Result<RunResponse, RunnerError> {
        let mut runtime = Runtime::new(wasm_fn.clone());
        match runtime.run() {
            Ok(r) => Ok(RunResponse::new(r.to_string())),
            Err(e) => Err(RunnerError::Execution(e.to_string())),
        }
    }
}
