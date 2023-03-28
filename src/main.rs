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

    stdout_log("👨🏻‍🏭 Start Writer executor").await?;
    let (writer_tx, writer) = Writer::new(db.clone());
    let writer_task = {
        let writer = Arc::clone(&writer);
        tokio::spawn(async move {
            writer.lock().await.run().await;
        })
    };

    stdout_log("👨🏼‍🔧 Start Reader executor").await?;
    let (reader_tx, reader) = Reader::new(db.clone());
    let reader_task = {
        let reader = Arc::clone(&reader);
        tokio::spawn(async move {
            reader.lock().await.run().await;
        })
    };
    stdout_log("👩🏾‍🔬 Start Runner executor").await?;
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
