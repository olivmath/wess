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

use self::models::{WriteJob, WriteOps};
use crate::{config::CONFIG, database::RocksDB};
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
    rx: Receiver<WriteJob>,
    db: RocksDB,
}

impl Writer {
    // # Creates a new instance of [`Writer`] with the given `db` instance.
    ///
    /// Returns a tuple containing a [`Sender<WriteJob>`] and an [`Arc<Mutex<Writer>>`] instance.
    pub fn new(db: RocksDB, tx_reader: Sender<String>) -> (Sender<WriteJob>, Arc<Mutex<Writer>>) {
        let channel_size = CONFIG.writer.channel_size;
        let (tx, rx) = mpsc::channel::<WriteJob>(channel_size);
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
            let id = job.id.clone();
            let send_reader = self.tx.clone();

            match job.write_op {
                WriteOps::Create => {
                    if let Err(e) = self.db.add(&id, job.write_module.expect("not found item")) {
                        log_error!(e.to_string(), e.status.into());
                    }
                }
                WriteOps::Update => match self.db.upd(&id, job.write_module.unwrap()) {
                    Ok(id) => {
                        spawn(async move { send_reader.send(id).await });
                    }
                    Err(e) => {
                        log_error!(e.to_string(), e.status.into());
                    }
                },

                WriteOps::Delete => match self.db.del(&id) {
                    Ok(id) => {
                        spawn(async move { send_reader.send(id).await });
                    }
                    Err(e) => {
                        log_error!(e.to_string(), e.status.into());
                    }
                },
            }
        }
    }
}
