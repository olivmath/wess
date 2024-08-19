//! # Wess: A WebAssembly Execution and Storage Service
//!
//! Wess is a high-level WebAssembly execution and storage service designed to provide
//! efficient and easy-to-use WebAssembly function execution. The service allows you to
//! store and run WebAssembly modules with just a few lines of code, abstracting away the
//! complexities of compilation, linking, and module instantiation.
//!
//! Key Features:
//!
//! - **Efficient Execution**: Wess provides a streamlined execution environment for
//!   running WebAssembly functions, reducing overhead and improving performance.
//! - **Caching**: Wess leverages a caching system to store frequently accessed data
//!   in memory, improving the overall performance of the service.
//! - **Concurrency**: The service is built with concurrency in mind, allowing for
//!   efficient execution of multiple tasks simultaneously.
//! - **Storage**: Wess uses RocksDB for storing WebAssembly modules, providing a
//!   fast and reliable storage solution.
//!
//! The main components of Wess include:
//!
//! - `database`: The module responsible for managing data storage using RocksDB.
//! - `logger`: A utility module for handling logs and terminal output.
//! - `server`: The module that implements the Wess server and API endpoints.
//! - `workers`: A set of modules that manage the execution of WebAssembly functions,
//!   including reader, writer, and runner.
//!
//! To get started with Wess

#[macro_use]
mod logger;

mod config;
mod database;
mod errors;
mod metrics;
mod server;
mod workers;

use crate::{config::CONFIG, database::RocksDB, metrics::collect_usage_metrics};
use log::info;
use logger::init_logger;
use server::WessServer;
use std::{error::Error, sync::Arc};
use tokio::{
    sync::{mpsc, Mutex},
    try_join,
};
use workers::{reader::Reader, runner::Runner, writer::Writer};

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logger();
    tokio::spawn(collect_usage_metrics());

    let config = Arc::clone(&CONFIG);

    info!("------------------------------------------------");
    info!("Starting Wess");
    info!("------------------------------------------------");

    info!("Start RocksDB data base");
    let db = RocksDB::new();

    let (tx_writer, rx_writer) = mpsc::channel::<String>(1);
    info!("Start Writer executor");
    let (writer_tx, writer) = Writer::new(db.clone(), tx_writer);
    let writer_task = {
        let writer = Arc::clone(&writer);
        tokio::spawn(async move {
            writer.lock().await.run().await;
        })
    };

    info!("Start Reader executor");
    let (reader_tx, reader) = Reader::new(db.clone(), rx_writer);
    let reader_task = {
        let reader = Arc::clone(&reader);
        tokio::spawn(async move {
            reader.lock().await.run().await;
        })
    };
    info!("Start Runner executor");
    let (runner_tx, runner) = Runner::new(db);
    let runner_task = {
        let runner = Arc::clone(&runner);
        tokio::spawn(async move {
            runner.lock().await.run().await;
        })
    };

    let addr = format!("{}:{}", config.server.address, config.server.port);
    info!("Start server on {}", &addr);
    let wess = Arc::new(Mutex::new(WessServer::new(writer_tx, reader_tx, runner_tx)));

    let server_task = {
        let wess = Arc::clone(&wess);
        tokio::spawn(async move {
            let wess_instance = wess.lock().await.clone();
            wess_instance.run(&addr).await.unwrap();
        })
    };

    try_join!(
        server_task,
        runner_task,
        writer_task,
        reader_task
    )
    .unwrap();
    Ok(())
}
