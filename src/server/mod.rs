//! This module contains the WessServer implementation and associated functionality.
//!
//! It provides an HTTP server built on top of the Tide framework to handle incoming requests,
//! which are then processed accordingly based on the registered routes.

pub mod request_handlers;
pub mod routes;

use self::routes::all_wasm;
use tide::Server;

/// WessServer is a struct that encapsulates the Tide server instance.
pub struct WessServer {
    app: Server<AppState>,
}

/// AppState represents the shared state of the application, including the RocksDB instance.
#[derive(Clone)]
struct AppState {
    _rocks_db: Arc<RocksDB>,
}

impl WessServer {
    /// Constructs a new WessServer with the provided RocksDB instance.
    ///
    /// # Arguments
    ///
    /// * `db` - A RocksDB instance to be used for the server's state.
    pub fn new(db: RocksDB) -> WessServer {
        let mut app = tide::with_state(AppState {
            _rocks_db: db.into(),
        });

        app.at("/").get(|_| async { all_wasm() });

        WessServer { app }
    }

    /// Starts the HTTP server and listens for incoming connections.
    ///
    /// This version of WessServer supports the following routes:
    /// * `GET "/"`: Responds with a status code of 200 (OK) with a list of all Wasms saved.
    pub async fn run(self, addr: &str) -> std::io::Result<()> {
        self.app.listen(addr).await
    }
}


// mod.rs (parte do arquivo)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::rocksdb::RocksDB;
    use async_std::task;
    use tide_testing::TideTestingExt;

    #[test]
    fn test_wess_server_new() {
        let db = RocksDB::new();
        let _server = WessServer::new(db);
    }

    #[test]
    fn test_wess_server_run() {
        task::block_on(async {
            let db = RocksDB::new();
            let server = WessServer::new(db);
            let client = server.app.client();
    
            let server_task = task::spawn(async move {
                server.run("127.0.0.1:777").await.unwrap();
            });
    
            let resp = client.get("/").send().await.unwrap();
            assert_eq!(resp.status(), StatusCode::Ok);
    
            server_task.cancel().await;
        });
    }
}
