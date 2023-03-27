//! # Contains the implementation of the server routes used to handle incoming requests.
//!
//! The routes are implemented as asynchronous functions that take a [`Request`]
//! object and return a [`Result`] containing the HTTP response.
//!
//!
//! The server routes are implemented as follows:
//!
//! * [`write_op`] - handles write operations (create, update, delete) on a WebAssembly function.
//! * [`read_op`] - handles read operations (get) on a WebAssembly function.
//! * [`run_op`] - handles requests to run a WebAssembly function.
//!
//! All routes take a [`Request`] object that provides access to the HTTP request data and a
//! [`AppState`] object that contains the application state (i.e., the channels used to communicate
//! with the worker threads).
//!
//! The [`write_op`] function is used to create, update or delete a WebAssembly function.
//! It extracts the request data and passes it to the [`send_to_writer`] function to be
//! sent to the writer thread.
//!
//! The [`read_op`] function is used to get the WebAssembly bytecode for a given function ID.
//! It passes the ID to the [`send_to_reader`] function to be sent to the reader thread.
//!
//! The [`run_op`] function is used to run a WebAssembly function. It extracts the request data and
//! passes it to the [`send_to_runner`] function to be sent to the runner thread.

/// Module containing functions for handling server routes.
mod utils;

use self::utils::{
    get_random_id, respond_with_error, send_to_reader, send_to_runner, send_to_writer,
};
use super::{
    models::{RunRequest, WRequest, WasmRequest},
    AppState,
};
use crate::workers::writer::models::WOps;
use tide::{Error, Request, Response};

/// # Creates a [`WasmRequest`] object from a [`Request`].
///
/// ## Arguments
///
/// * `req` - A mutable reference to the [`Request`] object to create the [`WasmRequest`] from.
/// * `wtype` - The type of write operation to perform.
///
/// ## Returns
///
/// A [`Result`] containing a tuple with the ID of the [`WasmFn`] object and the [`WRequest`] object.
///
/// # Errors
///
/// * Returns an [`Error`] object if there was an error parsing the JSON in the request.
async fn create_wasm_request(
    req: &mut Request<AppState>,
    wtype: &WOps,
) -> Result<(String, WRequest), Error> {
    match wtype {
        WOps::Create => {
            let wreq = req
                .body_json::<WasmRequest>()
                .await
                .map_err(|e| format!("invalid json {e}, {:?}", WasmRequest::default()))
                .unwrap();
            let id = get_random_id(&req, wreq.wasm.as_slice());
            Ok((id, WRequest(Some(wreq))))
        }
        WOps::Update => {
            let wreq = req
                .body_json::<WRequest>()
                .await
                .map_err(|_| "invalid json".to_string())
                .unwrap();
            let id = req.param("id")?.to_string();
            Ok((id, wreq))
        }
        WOps::Delete => {
            let id = req.param("id")?.to_string();
            Ok((id, WRequest(None)))
        }
    }
}

/// # Handler function for write operations.
///
/// ## Arguments
///
/// * `req` - The [`Request`] object containing the write operation to perform.
/// * `wtype` - The type of write operation to perform.
///
/// # Returns
///
/// A [`Result`] containing the [`Response`] object.
pub async fn write_op(mut req: Request<AppState>, wtype: WOps) -> Result<Response, Error> {
    let tx = req.state().writer_tx.clone();

    match create_wasm_request(&mut req, &wtype).await {
        Ok((id, wreq)) => send_to_writer(id, wreq, wtype, tx).await,
        Err(e) => respond_with_error(e.to_string()).await,
    }
}

/// # Handler function for read operations.
///
/// ## Arguments
///
/// * `req` - The [`Request`] object containing the read operation to perform.
///
/// ## Returns
///
/// A [`Result`] containing the [`Response`] object.
pub async fn read_op(req: Request<AppState>) -> Result<Response, Error> {
    let id = req.param("id").unwrap();
    let tx = req.state().reader_tx.clone();

    send_to_reader(id.to_owned(), tx).await
}

/// # Handler function for run operations.
///
/// ## Arguments
///
/// * `req` - The [`Request`] object containing the run operation to perform.
///
/// ## Returns
///
/// A [`Result`] containing the [`Response`] object.
pub async fn run_op(mut req: Request<AppState>) -> Result<Response, Error> {
    if let Ok(args) = req.body_json::<RunRequest>().await {
        let id = req.param("id").unwrap();
        let tx = req.state().runner_tx.clone();

        send_to_runner(id.to_owned(), args, tx).await
    } else {
        respond_with_error("invalid args".to_string()).await
    }
}
