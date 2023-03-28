//! # The `reader` module provides an async executor for reading data from the database
//!
//! This module contains the following main components:
//!
//! - [`Reader`]: A struct representing the async executor.
//! - [`Cache`]: A struct representing the in-memory cache for storing frequently accessed data.
//!
//! The `reader` module depends on the following modules:
//!
//! - [`models`]: A module that contains the models for wrapping data sent through channels.
//! - [`cache`]: A module that contains the cache implementation.
//!
//! The `reader` module provides an asynchronous interface for reading data from the database
//! and uses an in-memory cache to improve performance for frequently accessed data.

pub mod cache;
pub mod models;

use self::{
    cache::Cache,
    models::{RJob, ReadResponse},
};
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
    /// Cache instance for reading values from the memory cache.
    cache: Cache,
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
        let cache = Cache::new();
        (tx, Arc::new(Mutex::new(Reader { rx, db, cache })))
    }

    /// Runs the worker, listening for read requests on the channel receiver.
    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let responder = job.responder;
            let db_instance = &self.db;
            let id = job.id;
            //
            match self.cache.get(id.clone(), || db_instance.get(id.as_str())) {
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
