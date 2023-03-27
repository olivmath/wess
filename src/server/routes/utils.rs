use crate::{
    logger::log_err,
    server::{
        request::{RunRequest, WRequest},
        AppState,
    },
    workers::{
        reader::models::{RJob, ReadResponse},
        runner::models::{RunJob, RunResponse},
        writer::models::{WJob, WOps},
    },
};
use async_std::task;
use rand::Rng;
use serde::Serialize;
use serde_json::json;
use sha256::digest;
use tide::{Error, Request, Response, StatusCode};
use tokio::sync::{mpsc::Sender, oneshot};

/// # Generates a random id for the provided data or extracts the id from the request parameters.
///
/// ## Arguments
///
/// * `req` - The [`Request`] instance from which to extract the id.
/// * `data` - The data for which to generate the id.
///
/// ## Returns
///
/// * A [`String`] containing the extracted or generated id.
pub fn get_random_id(req: &Request<AppState>, data: &[u8]) -> String {
    req.param("id")
        .unwrap_or({
            let mut rng = rand::thread_rng();
            let random_number: u8 = rng.gen_range(0..100);

            digest([data, &[random_number, random_number]].concat().as_slice()).as_str()
        })
        .to_string()
}

/// # Returns an error response with the provided message.
///
/// ## Arguments
///
/// * `message` - The error message to include in the response body.
///
/// ## Returns
///
/// * A [`Result`] containing the error response.
pub async fn respond_with_error(message: String) -> Result<Response, Error> {
    Ok(Response::builder(StatusCode::InternalServerError)
        .body(json!({ "message": message }))
        .build())
}

/// # Returns a successful response with the provided message.
///
/// ## Arguments
///
/// * `message` - The message to include in the response body.
///
/// ## Returns
///
/// * A [`Result`] containing the successful response.
pub async fn respond<T>(message: T) -> Result<Response, Error>
where
    T: Serialize,
{
    Ok(Response::builder(StatusCode::Ok)
        .body(json!({ "message": message }))
        .build())
}

/// # Sends a [`WJob`] to the writer worker.
///
/// ## Arguments
///
/// * `id` - The id of the job.
/// * `wreq` - The [`WRequest`] instance to include in the job.
/// * `wtype` - The [`WOps`] type of the job.
/// * `tx` - The [`Sender`] instance used to send the job to the writer worker.
///
/// ## Returns
///
/// * A [`Result`] containing a successful response with the provided id.
pub async fn send_to_writer(
    id: String,
    wreq: WRequest,
    wtype: WOps,
    tx: Sender<WJob>,
) -> Result<Response, Error> {
    let wjob = WJob::new(wreq, wtype, id.clone());

    task::spawn(async move {
        if let Err(e) = tx.send(wjob).await {
            log_err(format!("Erro ao enviar mensagem: {e}"));
        }
    });

    respond(id).await
}

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
        Err(e) => respond_with_error(e.to_string()).await,
    }
}

/// # Sends a [`RunJob`] message to the `runner` worker to execute a WebAssembly function.
///
/// ## Arguments
///
/// * `id` - A [`String`] representing the ID of the WebAssembly function to execute.
/// * `args` - A [`RunRequest`] object containing the arguments to pass to the WebAssembly function.
/// * `tx` - A [`Sender`] for the `runner` worker's channel.
///
/// ## Returns
///
/// * A [`Result`] containing a [`Response`] with a serialized [`RunResponse`] message or an [`Error`]
/// response.
pub async fn send_to_runner(
    id: String,
    args: RunRequest,
    tx: Sender<RunJob>,
) -> Result<Response, Error> {
    let (done_tx, done_rx) = oneshot::channel::<RunResponse>();
    let run_job = RunJob::new(done_tx, args, id);

    tx.send(run_job).await.unwrap();

    match done_rx.await {
        Ok(response) => respond(response).await,
        Err(e) => respond_with_error(e.to_string()).await,
    }
}
