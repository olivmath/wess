//! This module contains the WessServer implementation and associated functionality.
//!
//! It provides an HTTP server built on top of the Tide framework to handle incoming requests,
//! which are then processed accordingly based on the registered routes.

pub mod request;
mod routes;

use self::routes::{get_wasm, job_maker};
use crate::{runner::job::Job, wasm::JobType};
use tide::Server;
use tokio::sync::mpsc::Sender;

/// AppState represents the shared state of the application, including the Sender<WasmJob> instance.
#[derive(Clone)]
pub struct AppState {
    pub tx: Sender<Job>,
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
    pub fn new(tx: Sender<Job>) -> WessServer {
        let mut app = tide::with_state(AppState { tx });
        app.at("/:id")
            .get(|req| async { get_wasm(req).await })
            .put(|req| async { job_maker(req, JobType::Modity).await })
            .delete(|req| async { job_maker(req, JobType::Delete).await });

        app.at("/")
            .post(|req| async { job_maker(req, JobType::Save).await });

        WessServer { app }
    }

    /// Starts the HTTP server and listens for incoming connections.
    ///
    /// This version of WessServer supports the following routes:
    pub async fn run(self, addr: &str) -> std::io::Result<()> {
        self.app.listen(addr).await
    }
}
