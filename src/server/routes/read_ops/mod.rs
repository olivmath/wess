use std::convert::TryInto;

use crate::{
    errors::WessError,
    metrics::constants::READER_CHANNEL_QUEUE,
    server::{
        response::{respond, respond_with_error},
        AppState,
    },
    workers::reader::models::{ReadJob, ReadResponse},
};
use tide::{Error, Request, Response};
use tokio::sync::{mpsc::Sender, oneshot};
use uuid::Uuid;

/// # Handler function for read operations.
///
/// ## Arguments
///
/// * `req` - The [`Request`] object containing the read operation to perform.
///
/// ## Returns
///
/// A [`Result`] containing the [`Response`] object.
pub async fn make_read_op(req: Request<AppState>) -> Result<Response, Error> {
    match validate_id(&req) {
        Ok(input) => {
            let reader_tx: Sender<ReadJob> = req.state().reader_tx.clone();
            match input {
                Some(id) => send_to_reader(id, reader_tx).await,
                None => get_all(reader_tx).await,
            }
        }
        Err(e) => respond_with_error(e).await,
    }
}

fn validate_id(req: &Request<AppState>) -> Result<Option<String>, WessError> {
    match req.param("id") {
        Ok(input) => match Uuid::parse_str(input) {
            Ok(id) => Ok(Some(id.to_string())),
            Err(e) => {
                let werr = log_error!(format!("Invalid ID: {}", e.to_string()), 400);
                Err(werr)
            }
        },
        Err(e) => {
            eprint!("hmmm");
            let werr = log_error!(e.to_string(), 400);
            Err(werr)
        }
    }
}

async fn get_all(reader_tx: Sender<ReadJob>) -> Result<Response, Error> {
    let (tx, rx) = oneshot::channel::<ReadResponse>();
    let read_job = ReadJob { id: None, tx };

    reader_tx.send(read_job).await.unwrap();
    READER_CHANNEL_QUEUE.set(reader_tx.capacity().try_into().unwrap());

    match rx.await {
        Ok(response) => match response {
            ReadResponse::Module(_) => todo!(),
            ReadResponse::Size(r) => respond(r).await,
            ReadResponse::Fail(e) => {
                let werr = log_error!(e.to_string(), 500);
                respond_with_error(werr).await
            }
        },
        Err(e) => {
            let werr = log_error!(e.to_string(), 500);
            respond_with_error(werr).await
        }
    }
}

/// # Sends a message to the reader worker to read a WebAssembly function.
///
/// ## Arguments
///
/// * `id`: A [`String`] containing the ID of the WebAssembly function to read.
/// * `reader_tx`: A [`Sender`] of [`ReadJob`] messages to send the job to the Reader worker.
///
/// ## Returns
///
/// * A [`Result`] containing the `Response` with the result of the read operation
/// or an [`Error`] if the operation failed.
pub async fn send_to_reader(id: String, reader_tx: Sender<ReadJob>) -> Result<Response, Error> {
    let (tx, rx) = oneshot::channel::<ReadResponse>();
    let job = ReadJob { id: Some(id), tx };

    reader_tx.send(job).await.unwrap();
    READER_CHANNEL_QUEUE.set(reader_tx.capacity() as i64);

    match rx.await {
        Ok(response) => match response {
            ReadResponse::Module(_) => todo!(),
            ReadResponse::Size(r) => respond(r).await,
            ReadResponse::Fail(e) => {
                let werr = log_error!(e.to_string(), 500);
                respond_with_error(werr).await
            }
        },
        Err(e) => {
            let werr = log_error!(e.to_string(), 500);
            respond_with_error(werr).await
        }
    }
}
