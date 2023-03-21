//! This module contains the WessServer implementation and associated functionality.
//!
//! It provides an HTTP server built on top of the Tide framework to handle incoming requests,
//! which are then processed accordingly based on the registered routes.

pub mod routes;

use self::routes::{add_new_wasm, get_wasm, remove_wasm, update_wasm};
use tide::Server;

/// WessServer is a struct that encapsulates the Tide server instance.
pub struct WessServer {
    app: Server<()>,
}

impl WessServer {
    /// Constructs a new WessServer with the provided RocksDB instance.
    ///
    /// # Arguments
    ///
    /// * `db` - A RocksDB instance to be used for the server's state.
    pub fn new() -> WessServer {
        let mut app = tide::new();

        app.at("/:id").get(|req| async { get_wasm(req) });
        app.at("/")
            .post(|req| async { add_new_wasm(req) })
            .put(|req| async { update_wasm(req) })
            .delete(|req| async { remove_wasm(req) });

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

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task;
    use tide::StatusCode;
    use tide_testing::TideTestingExt;

    #[test]
    fn test_wess_server_new() {
        let _server = WessServer::new();
    }

    #[test]
    fn test_wess_server_run() {
        task::block_on(async {
            let server = WessServer::new();
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
