use crate::{
    logger,
    server::{
        errors::RequestError,
        models::{WRequest, WasmRequest},
        response::{respond, respond_with_error},
        AppState,
    },
    workers::writer::models::{WJob, WOps},
};
use rand::Rng;
use sha256::digest;
use tide::{Error, Request, Response};
use tokio::{sync::mpsc::Sender, task};

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

/// # Creates a [`WasmRequest`] object from a [`Request`].
///
/// ## Arguments
///
/// * `req` - A mutable reference to the [`Request`] object to create the [`WasmRequest`] from.
/// * `wtype` - The type of write operation to perform.
///
/// ## Returns
///
/// A [`Result`] containing a tuple with the ID of the `WasmFn` object and the [`WRequest`] object.
///
/// # Errors
///
/// * Returns an [`Error`] object if there was an error parsing the JSON in the request.
async fn wrap_request(
    req: &mut Request<AppState>,
    wtype: &WOps,
) -> Result<(String, WRequest), Error> {
    match wtype {
        WOps::Create => {
            let wreq = req
                .body_json::<WasmRequest>()
                .await
                .map_err(|e| logger::log_error(RequestError::InvalidJson(e.to_string())))
                .unwrap();
            let id = get_random_id(&req, wreq.wasm.as_slice());
            Ok((id, WRequest(Some(wreq))))
        }
        WOps::Update => {
            let wreq = req
                .body_json::<WRequest>()
                .await
                .map_err(|e| logger::log_error(RequestError::InvalidJson(e.to_string())))
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
            logger::log_error(RequestError::ChannelError(e.to_string()));
        }
    });

    respond(id).await
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
pub async fn make_write_op(mut req: Request<AppState>, wtype: WOps) -> Result<Response, Error> {
    let tx = req.state().writer_tx.clone();

    match wrap_request(&mut req, &wtype).await {
        Ok((id, wreq)) => send_to_writer(id, wreq, wtype, tx).await,
        Err(e) => respond_with_error(e.to_string()).await,
    }
}
