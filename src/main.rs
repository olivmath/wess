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

mod database;
mod logger;
mod server;
mod workers;

use database::RocksDB;
use logger::{clear_terminal_with, stdout_log};
use server::WessServer;
use std::{error::Error, sync::Arc};
use tokio::{sync::Mutex, try_join};
use workers::{reader::Reader, runner::Runner, writer::Writer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    clear_terminal_with("");

    stdout_log("💽 Start RocksDB data base").await?;
    let db = RocksDB::new();

    stdout_log("🏗️ Create a threadset to run the tasks in the background").await?;

    stdout_log("👨‍🏭 Start Writer executor").await?;
    let (writer_tx, writer) = Writer::new(db.clone());
    let writer_task = {
        let writer = Arc::clone(&writer);
        tokio::spawn(async move {
            writer.lock().await.run().await;
        })
    };

    stdout_log("👨‍🔧 Start Reader executor").await?;
    let (reader_tx, reader) = Reader::new(db.clone());
    let reader_task = {
        let reader = Arc::clone(&reader);
        tokio::spawn(async move {
            reader.lock().await.run().await;
        })
    };
    stdout_log("👩‍🔬 Start Runner executor").await?;
    let (runner_tx, runner) = Runner::new(db);
    let runner_task = {
        let runner = Arc::clone(&runner);
        tokio::spawn(async move {
            runner.lock().await.run().await;
        })
    };

    stdout_log("🛰️  Run server on `http://127.0.0.1:3000`").await?;
    let wess = Arc::new(Mutex::new(WessServer::new(writer_tx, reader_tx, runner_tx)));

    let server_task = {
        let wess = Arc::clone(&wess);
        tokio::spawn(async move {
            let wess_instance = wess.lock().await.clone();
            wess_instance.run("127.0.0.1:3000").await.unwrap();
        })
    };

    try_join!(server_task, runner_task, writer_task, reader_task)?;
    Ok(())
}
