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

use self::models::{WJob, WOps, WriterError};
use crate::{config::CONFIG, database::RocksDB, logger};
use std::sync::Arc;
use tokio::{
    spawn,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

/// An async executor for writing data into the database.
pub struct Writer {
    tx: Sender<String>,
    rx: Receiver<WJob>,
    db: RocksDB,
}

impl Writer {
    // # Creates a new instance of [`Writer`] with the given `db` instance.
    ///
    /// Returns a tuple containing a [`Sender<WJob>`] and an [`Arc<Mutex<Writer>>`] instance.
    pub fn new(db: RocksDB, tx_reader: Sender<String>) -> (Sender<WJob>, Arc<Mutex<Writer>>) {
        let channel_size = CONFIG.writer.channel_size;
        let (tx, rx) = mpsc::channel::<WJob>(channel_size);
        (
            tx,
            Arc::new(Mutex::new(Writer {
                tx: tx_reader,
                rx,
                db,
            })),
        )
    }

    /// # Runs the async executor for writing data into the database.
    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let job_type = job.wtype;
            let wasm_req = job.wreq;
            let id = job.id.as_str();
            let send_reader = self.tx.clone();
            //
            match job_type {
                WOps::Create => {
                    if let Err(e) = self.db.add(id, wasm_req.unwrap().to_wasm_fn().unwrap()) {
                        logger::log_error(WriterError::Create(id.to_string(), e.to_string()));
                    }
                }
                WOps::Update => match self.db.upd(id, wasm_req.unwrap().to_wasm_fn().unwrap()) {
                    Ok(id) => {
                        spawn(async move { send_reader.send(id).await });
                    }
                    Err(e) => {
                        logger::log_error(WriterError::Update(id.to_string(), e.to_string()));
                    }
                },
                WOps::Delete => match self.db.del(id) {
                    Ok(id) => {
                        spawn(async move { send_reader.send(id).await });
                    }
                    Err(e) => {
                        logger::log_error(WriterError::Delete(id.to_string(), e.to_string()));
                    }
                },
            }
        }
    }
}
