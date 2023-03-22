//! This module contains the WessServer implementation and associated functionality.
//!
//! It provides an HTTP server built on top of the Tide framework to handle incoming requests,
//! which are then processed accordingly based on the registered routes.

pub mod request;
mod routes;

use self::routes::{add_new_wasm, get_wasm, remove_wasm, update_wasm};
use tide::Server;
/// AppState represents the shared state of the application, including the Sender<WasmJob> instance.
#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<WasmJob>,
}

/// WessServer is a struct that encapsulates the Tide server instance.
pub struct WessServer {
    app: Server<AppState>,
}

impl WessServer {
    /// Constructs a new WessServer with the provided Sender<WasmJob> instance.
    ///
    /// # Arguments
    ///
    /// * `tx` - A Sender<WasmJob> instance to be used for the server's state.
    #[allow(clippy::new_without_default)]
    pub fn new(tx: Sender<WasmJob>) -> WessServer {
        let mut app = tide::with_state(AppState { tx });

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
    pub async fn run(self, addr: &str) -> std::io::Result<()> {
        self.app.listen(addr).await
    }
}
