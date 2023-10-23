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
    models::{ReadJob, ReadResponse},
};
use crate::{config::CONFIG, database::RocksDB};
use std::sync::Arc;
use tokio::{
    select,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

/// Worker responsible for reading values from the database.
pub struct Reader {
    /// Channel receiver that receives read requests.
    rx: Receiver<ReadJob>,
    /// Database instance to read values from.
    db: RocksDB,
    /// Cache instance for reading values from the memory cache.
    cache: Cache,
    rx_writer: Receiver<String>,
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
    pub fn new(db: RocksDB, rx_writer: Receiver<String>) -> (Sender<ReadJob>, Arc<Mutex<Reader>>) {
        let channel_size = CONFIG.reader.channel_size;
        let (tx, rx) = mpsc::channel::<ReadJob>(channel_size);
        let cache = Cache::new();
        (
            tx,
            Arc::new(Mutex::new(Reader {
                rx,
                rx_writer,
                db,
                cache,
            })),
        )
    }

    /// Runs the worker, listening for read requests on the channel receiver.
    pub async fn run(&mut self) {
        loop {
            select! {
                Some(id) = self.rx_writer.recv() => {
                    self.cache.del(id)
                },
                Some(job) = self.rx.recv() => {
                    let option_id = job.id.clone();
                    let tx = job.tx;
                    //
                    match option_id {
                        None => {
                            let r = self.db.all().len().clone();
                            tokio::spawn(async move {
                                tx.send(ReadResponse::Size(r))
                            });
                        },
                        Some(id) => {
                            let db = &self.db;
                            let f = |i: &str| db.get(i);
                            let cache_result = self.cache.get(&id, f);

                            match cache_result {
                                Some(wasm_module) => {
                                    tokio::spawn(async move {
                                        tx.send(ReadResponse::Module(wasm_module))
                                    });
                                }
                                None => {
                                    tokio::spawn(
                                        async move {
                                        let werr = log_error!("Not found".to_string(), 404);
                                        tx.send(ReadResponse::Fail(werr)) },
                                    );
                                }
                            };
                        }
                    }
                }
            }
        }
    }
}
