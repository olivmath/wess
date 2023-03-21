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
    stdout_log("💽 start RocksDB data base").await?;
    let db = RocksDB::new();
    stdout_log("🚿 Create channels for HTTP requests").await?;

    stdout_log("👷 Criar um conjunto de threads para executar as tarefas em segundo plano").await?;
    stdout_log("📡 Enviar as tarefas recebidas para o canal de tarefas dos runners").await?;
    stdout_log("🛰️ Configurar o servidor usando o framework Tide").await?;
    let wess = WessServer::new(db);
    wess.run("127.0.0.1:3000").await?;

    Ok(())
}
