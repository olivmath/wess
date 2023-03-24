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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    clear_terminal_with("");

    stdout_log("💽 Start RocksDB data base").await?;
    let db = RocksDB::new();

    stdout_log("🚿 Create channels for HTTP requests").await?;
    let (tx, rx) = mpsc::channel::<Job>(100);

    stdout_log("👷 Create a threadset to run the tasks in the background").await?;
    let runner = Arc::new(Mutex::new(Runner::new(rx, db)));

    stdout_log("🛰️  Run server on `http://127.0.0.1:3000`").await?;
    let wess = Arc::new(Mutex::new(WessServer::new(tx)));

    stdout_log("📡 Send the received tasks to the runners' tasks channel").await?;

    let runner_task = {
        let runner = Arc::clone(&runner);
        tokio::spawn(async move {
            runner.lock().await.run().await;
        })
    };

    let server_task = {
        let wess = Arc::clone(&wess);
        tokio::spawn(async move {
            let wess_instance = wess.lock().await.clone();
            wess_instance.run("127.0.0.1:3000").await.unwrap();
        })
    };

    try_join!(server_task, runner_task)?;
    Ok(())
}
