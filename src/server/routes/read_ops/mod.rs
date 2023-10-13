use std::convert::TryInto;

use crate::{
    metrics::constants::READER_CHANNEL_QUEUE,
    server::{
        errors::RequestError,
        response::{respond, respond_with_error},
        AppState,
    },
    workers::reader::models::{ReadJob, ReadResponse},
};
use tide::{Error, Request, Response, StatusCode};
use tokio::sync::{mpsc::Sender, oneshot};

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
    let reader_tx = req.state().reader_tx.clone();
    if req.url().path() == "/" {
        return get_all(reader_tx).await;
    }
    let id = req.param("id").unwrap();

    send_to_reader(id.to_owned(), reader_tx).await
}

async fn get_all(reader_tx: Sender<ReadJob>) -> Result<Response, Error> {
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();
    let read_job = ReadJob::new(done_tx, "".to_string());

    reader_tx.send(read_job).await.unwrap();
    READER_CHANNEL_QUEUE.set(reader_tx.capacity().try_into().unwrap());

    match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Success(r) => respond(r).await,
            ReadResponse::Fail(e) => {
                respond_with_error(log_error!(e).to_string(), StatusCode::InternalServerError).await
            }
        },
        Err(e) => {
            log_error!(RequestError::ChannelError(e.to_string()));
            respond_with_error(e.to_string(), StatusCode::InternalServerError).await
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
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();
    let read_job = ReadJob::new(done_tx, id);

    reader_tx.send(read_job).await.unwrap();
    READER_CHANNEL_QUEUE.set(reader_tx.capacity() as i64);

    match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Success(r) => respond(r).await,
            ReadResponse::Fail(e) => {
                respond_with_error(log_error!(e).to_string(), StatusCode::InternalServerError).await
            }
        },
        Err(e) => {
            log_error!(RequestError::ChannelError(e.to_string()));
            respond_with_error(e.to_string(), StatusCode::InternalServerError).await
        }
    }
}
