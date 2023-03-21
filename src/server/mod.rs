//! This module contains the WessServer implementation and associated functionality.
//!
//! It provides an HTTP server built on top of the Tide framework to handle incoming requests,
//! which are then processed accordingly based on the registered routes.

pub mod request_handlers;
pub mod routes;
pub mod tls;

use super::database::rocksdb::RocksDB;
use std::sync::Arc;
use tide::Server;
use tide::{Response, StatusCode};

/// WessServer is a struct that encapsulates the Tide server instance.
pub struct WessServer {
    app: Server<AppState>,
}

/// AppState represents the shared state of the application, including the RocksDB instance.
#[derive(Clone)]
struct AppState {
    rocks_db: Arc<RocksDB>,
}

impl WessServer {
    /// Constructs a new WessServer with the provided RocksDB instance.
    ///
    /// # Arguments
    ///
    /// * `db` - A RocksDB instance to be used for the server's state.
    pub fn new(db: RocksDB) -> WessServer {
        let mut app = tide::with_state(AppState {
            rocks_db: db.into(),
        });

        app.at("/")
            .get(|_| async { Ok(Response::new(StatusCode::Ok)) });

        WessServer { app }
    }

    /// Starts the HTTP server and listens for incoming connections.
    ///
    /// This version of WessServer supports the following routes:
    /// * `GET "/"`: Responds with a status code of 200 (OK) without any additional content.
    pub async fn run(self) -> std::io::Result<()> {
        self.app.listen("127.0.0.1:3000").await
    }
}
