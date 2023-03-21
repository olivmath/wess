mod database;
mod logger;
pub mod server;
mod wasm;

use database::rocksdb::RocksDB;
use logger::stdout_log;
use server::WessServer;
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    stdout_log("💽 Start RocksDB data base").await?;
    let db = RocksDB::new();

    stdout_log("🚿 Create channels for HTTP requests").await?;
    stdout_log("👷 Create a threadset to run the tasks in the background").await?;
    stdout_log("📡 Send the received tasks to the runners' tasks channel").await?;

    stdout_log("🛰️  Run server on `http://127.0.0.1:3000`").await?;
    let wess = WessServer::new(db);
    wess.run("127.0.0.1:3000").await?;

    Ok(())
}
