use crate::{
    logger,
    server::{
        errors::RequestError,
        models::WRequest,
        response::{respond, respond_with_error},
        AppState,
    },
    workers::{
        reader::models::{RJob, ReadResponse},
        writer::models::{WJob, WOps},
    },
};
use async_std::task;
use rand::Rng;
use sha256::digest;
use tide::{Error, Request, Response};
use tokio::sync::{mpsc::Sender, oneshot};

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
    let tx = req.state().reader_tx.clone();
    match wtype {
        WOps::Create => {
            let id = random_id();
            let wreq = deserialize_request(&mut req).await.unwrap();
            perform_op(Some(wreq), id, wtype, req.state().writer_tx.clone()).await
        }
        WOps::Update => match verify_id(&req, tx).await {
            Ok(id) => {
                let wreq = deserialize_request(&mut req).await.unwrap();
                perform_op(Some(wreq), id, wtype, req.state().writer_tx.clone()).await
            }
            Err(e) => respond_with_error(e.to_string()).await,
        },
        WOps::Delete => match verify_id(&req, tx).await {
            Ok(id) => perform_op(None, id, wtype, req.state().writer_tx.clone()).await,
            Err(e) => respond_with_error(e.to_string()).await,
        },
    }
}
async fn perform_op(
    wreq: Option<WRequest>,
    id: String,
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
async fn verify_id(req: &Request<AppState>, tx: Sender<RJob>) -> Result<String, RequestError> {
    let id = req.param("id").unwrap();
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();
    let rjob = RJob::new(done_tx, id.to_string());

    tx.send(rjob).await.unwrap();

    match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Success(_) => Ok(id.to_string()),
            ReadResponse::Fail(e) => Err(RequestError::InvalidId(e)),
        },
        Err(e) => {
            logger::log_error(RequestError::ChannelError(e.to_string()));
            Err(RequestError::ChannelError(e.to_string()))
        }
    }
}

async fn deserialize_request(req: &mut Request<AppState>) -> Result<WRequest, RequestError> {
    Ok(req
        .body_json::<WRequest>()
        .await
        .map_err(|e| logger::log_error(RequestError::InvalidJson(e.to_string())))
        .unwrap())
}

fn random_id() -> String {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..u32::MAX);

    digest(random_number.to_be_bytes().as_slice())
}
