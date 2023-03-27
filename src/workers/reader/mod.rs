//! # The `reader` module provides a async executor for read data from database
//!
//! This module contains the following main components:
//!
//! - [`Reader`]: A struct representing the async executor.
//!
//! The `reader` module depends on the following modules:
//!
//! - [`models`]: A module that contains the models for wrap data by channels.

pub mod models;

use self::models::{RJob, ReadResponse};
use crate::database::RocksDB;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

/// Worker responsible for reading values from the database.
pub struct Reader {
    /// Channel receiver that receives read requests.
    rx: Receiver<RJob>,
    /// Database instance to read values from.
    db: RocksDB,
}

impl Reader {
    /// # Creates a new instance of [`Reader`].
    ///
    /// ## Arguments
    ///
    /// * `db` - The [`RocksDB`] instance to read values from.
    ///
    /// ## Returns
    ///
    /// A tuple containing a channel sender and an [`Arc<Mutex<Reader>>`] instance.
    pub fn new(db: RocksDB) -> (Sender<RJob>, Arc<Mutex<Reader>>) {
        let (tx, rx) = mpsc::channel::<RJob>(100);
        (tx, Arc::new(Mutex::new(Reader { rx, db })))
    }

    /// Runs the worker, listening for read requests on the channel receiver.
    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let responder = job.responder;
            let id = job.id;
            //
            match self.db.get(id.as_str()) {
                Some(wasm_fn) => {
                    tokio::spawn(async move { responder.send(ReadResponse::new(wasm_fn)) });
                }
                None => {
                    tokio::spawn(async move {
                        responder.send(ReadResponse::fail("wasm fn not found".into()))
                    });
                }
            };
        }
    }
}
