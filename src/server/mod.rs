//! # The `server` module provides a HTTP server for the Wess application.
//!
//! This module contains the following main components:
//!
//! - [`WessServer`]: A struct representing the HTTP server.
//! - [`AppState`]: A struct representing the global state of the server.
//!
//! The `server` module depends on the following modules:
//!
//! - [`routes`]: A module that contains the logic for handling HTTP routes.
//! - [`models`]: A module that contains the models for wrap json requests.

pub mod models;
mod routes;

use self::routes::{read_op, run_op, write_op};
use crate::workers::{
    reader::models::RJob,
    runner::models::RunJob,
    writer::models::{WJob, WOps},
};
use tide::Server;
use tokio::sync::mpsc::Sender;

/// The global state for the WessServer,
/// which holds the necessary `Sender`s for writing,
/// reading and running jobs.
#[derive(Clone)]
pub struct AppState {
    pub writer_tx: Sender<WJob>,
    pub reader_tx: Sender<RJob>,
    pub runner_tx: Sender<RunJob>,
}

/// The main server struct for the Wess application.
#[derive(Clone)]
pub struct WessServer {
    app: Server<AppState>,
}

impl WessServer {
    /// # Creates a new instance of the WessServer struct.
    ///
    /// ## Arguments
    ///
    /// * `writer_tx` - A `Sender<WJob>` for sending jobs to the writer worker.
    /// * `reader_tx` - A `Sender<RJob>` for sending jobs to the reader worker.
    /// * `runner_tx` - A `Sender<RunJob>` for sending jobs to the runner worker.
    ///
    /// ## Returns
    ///
    /// * An instance of `WessServer`.
    pub fn new(
        writer_tx: Sender<WJob>,
        reader_tx: Sender<RJob>,
        runner_tx: Sender<RunJob>,
    ) -> Self {
        let mut app = tide::with_state(AppState {
            writer_tx,
            reader_tx,
            runner_tx,
        });

        // Writer ops
        app.at("/")
            .post(|req| async { write_op(req, WOps::Create).await });
        app.at("/:id")
            .put(|req| async { write_op(req, WOps::Update).await })
            .delete(|req| async { write_op(req, WOps::Delete).await });

        // Read ops
        app.at("/:id").get(|req| async { read_op(req).await });

        // Run Ops
        app.at("/:id").post(|req| async { run_op(req).await });

        WessServer { app }
    }

    /// # Starts the server on the specified address.
    ///
    /// ## Arguments
    ///
    /// * `addr` - A `&str` representing the address to listen to.
    ///
    /// ## Returns
    ///
    /// * A `std::io::Result` indicating if the server started successfully.
    pub async fn run(self, addr: &str) -> std::io::Result<()> {
        self.app.listen(addr).await
    }
}
