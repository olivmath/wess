use crate::{
    logger,
    server::{
        errors::RequestError,
        models::RunRequest,
        response::{respond, respond_with_error},
        AppState,
    },
    workers::runner::models::{RunJob, RunResponse},
};
use tide::{Error, Request, Response};
use tokio::sync::{mpsc::Sender, oneshot};

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
        Ok(response) => match response {
            RunResponse::Success(r) => respond(r).await,
            RunResponse::Fail(f) => respond_with_error(f.to_string()).await,
        },
        Err(e) => {
            logger::log_error(RequestError::ChannelError(e.to_string()));
            respond_with_error(e.to_string()).await
        }
    }
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
pub async fn make_run_op(mut req: Request<AppState>) -> Result<Response, Error> {
    if let Ok(args) = req.body_json::<RunRequest>().await {
        let id = req.param("id").unwrap();
        let tx = req.state().runner_tx.clone();

        send_to_runner(id.to_owned(), args, tx).await
    } else {
        let e = logger::log_error(RequestError::InvalidJson("invalid args".to_string()));
        respond_with_error(e.to_string()).await
    }
}
