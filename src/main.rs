mod logger;

use std::error::Error;
use logger::stdout_log;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    stdout_log("💽 Configurar o banco de dados RocksDB").await?;
    stdout_log("🚿 Criar um canal de tarefas para receber as solicitações HTTP").await?;
    stdout_log("👷 Criar um conjunto de threads para executar as tarefas em segundo plano").await?;
    stdout_log("📡 Enviar as tarefas recebidas para o canal de tarefas dos runners").await?;
    stdout_log("🛰️ Configurar o servidor usando o framework Tide").await?;

    Ok(())
}
