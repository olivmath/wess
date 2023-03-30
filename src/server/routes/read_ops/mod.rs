use crate::{
    logger,
    server::{
        errors::RequestError,
        response::{respond, respond_with_error},
        AppState,
    },
    workers::reader::models::{RJob, ReadResponse},
};
use tide::{Error, Request, Response};
use tokio::sync::{mpsc::Sender, oneshot};

/// # Sends a message to the reader worker to read a WebAssembly function.
///
/// ## Arguments
///
/// * `id`: A [`String`] containing the ID of the WebAssembly function to read.
/// * `tx`: A [`Sender`] of [`RJob`] messages to send the job to the reader worker.
///
/// ## Returns
///
/// * A [`Result`] containing the `Response` with the result of the read operation
/// or an [`Error`] if the operation failed.
pub async fn send_to_reader(id: String, tx: Sender<RJob>) -> Result<Response, Error> {
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();
    let rjob = RJob::new(done_tx, id);

    tx.send(rjob).await.unwrap();

    match done_rx.await {
        Ok(response) => respond(response).await,
        Err(e) => {
            logger::log_error(RequestError::ChannelError(e.to_string()));
            respond_with_error(e.to_string()).await
        }
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
pub async fn make_read_op(req: Request<AppState>) -> Result<Response, Error> {
    let id = req.param("id").unwrap();
    let tx = req.state().reader_tx.clone();

    send_to_reader(id.to_owned(), tx).await
}
