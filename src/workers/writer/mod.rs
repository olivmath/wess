//! # The `writer` module provides a async executor for write data into database
//!
//! This module contains the following main components:
//!
//! - [`Writer`]: A struct representing the async executor.
//!
//! The `writer` module depends on the following modules:
//!
//! - [`models`]: A module that contains the models for wrap data by channels.

pub mod models;

use self::models::{WJob, WOps};
use crate::{database::RocksDB, logger::log_err};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

/// An async executor for writing data into the database.
pub struct Writer {
    rx: Receiver<WJob>,
    db: RocksDB,
}

impl Writer {
    // # Creates a new instance of [`Writer`] with the given `db` instance.
    ///
    /// Returns a tuple containing a [`Sender<WJob>`] and an [`Arc<Mutex<Writer>>`] instance.
    pub fn new(db: RocksDB) -> (Sender<WJob>, Arc<Mutex<Writer>>) {
        let (tx, rx) = mpsc::channel::<WJob>(100);
        (tx, Arc::new(Mutex::new(Writer { rx, db })))
    }

    /// # Runs the async executor for writing data into the database.
    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let job_type = job.wtype;
            let wasm_req = job.wreq;
            let id = job.id.as_str();
            //
            match job_type {
                WOps::Create => {
                    if let Err(e) = self.db.add(id, wasm_req.to_wasm_fn().unwrap()) {
                        // TODO: create a logger for audit errors
                        log_err(format!("Erro ao criar id: {id} erro: {}", e.to_string()));
                    };
                }
                WOps::Update => {
                    if let Err(e) = self.db.upd(id, wasm_req.to_wasm_fn().unwrap()) {
                        // TODO: create a logger for audit errors
                        log_err(format!("Erro ao editar id: {id} erro: {}", e.to_string()));
                    };
                }
                WOps::Delete => {
                    if let Err(e) = self.db.del(id) {
                        // TODO: create a logger for audit errors
                        log_err(format!("Erro ao deletar id: {id} erro: {}", e.to_string()));
                    };
                }
            }
        }
    }
}
