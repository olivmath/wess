mod database;
mod logger;
mod runner;
mod server;
mod wasm;

use crate::runner::Runner;
use database::RocksDB;
use logger::{clear_terminal_with, stdout_log};
use runner::job::Job;
use server::WessServer;
use std::{error::Error, sync::Arc};
use tokio::{
    sync::{mpsc, Mutex},
    try_join,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    clear_terminal_with("");

    stdout_log("💽 Start RocksDB data base").await?;
    let _db = RocksDB::new();

    stdout_log("🚿 Create channels for HTTP requests").await?;
    let (tx, rx) = mpsc::channel::<Job>(100);

    stdout_log("🛰️  Run server on `http://127.0.0.1:3000`").await?;
    let wess = WessServer::new(tx);
    wess.run("127.0.0.1:3000").await;

    stdout_log("👷 Create a threadset to run the tasks in the background").await?;
    stdout_log("📡 Send the received tasks to the runners' tasks channel").await?;

    Ok(())
}
