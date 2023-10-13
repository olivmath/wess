use std::convert::TryInto;

use crate::{
    database::models::WasmModule,
    metrics::constants::WRITER_CHANNEL_QUEUE,
    server::{
        errors::RequestError,
        response::{respond, respond_with_error},
        AppState,
    },
    workers::{
        reader::models::{ReadJob, ReadResponse},
        writer::models::{WriteJob, WriteOps},
    },
};
use async_std::task;
use rand::Rng;
use sha256::digest;
use tide::{Error, Request, Response, StatusCode};
use tokio::sync::{mpsc::Sender, oneshot};

/// # Handler function for write operations.
///
/// ## Arguments
///
/// * `req` - The [`Request`] object containing the write operation to perform.
/// * `write_type` - The type of write operation to perform.
///
/// # Returns
///
/// A [`Result`] containing the [`Response`] object.
pub async fn make_write_op(
    mut req: Request<AppState>,
    write_type: WriteOps,
) -> Result<Response, Error> {
    let reader_tx = req.state().reader_tx.clone();
    match write_type {
        WriteOps::Create => {
            let id = random_id();
            match deserialize_request(&mut req).await {
                Ok(write_request) => {
                    send_to_writer(
                        Some(write_request),
                        id,
                        write_type,
                        req.state().writer_tx.clone(),
                    )
                    .await
                }
                Err(e) => respond_with_error(e.to_string(), StatusCode::BadRequest).await,
            }
        }
        WriteOps::Update => match verify_id(&req, reader_tx).await {
            Ok(id) => match deserialize_request(&mut req).await {
                Ok(write_request) => {
                    send_to_writer(
                        Some(write_request),
                        id,
                        write_type,
                        req.state().writer_tx.clone(),
                    )
                    .await
                }
                Err(e) => respond_with_error(e.to_string(), StatusCode::BadRequest).await,
            },
            Err(e) => respond_with_error(e.to_string(), StatusCode::BadRequest).await,
        },
        WriteOps::Delete => match verify_id(&req, reader_tx).await {
            Ok(id) => send_to_writer(None, id, write_type, req.state().writer_tx.clone()).await,
            Err(e) => respond_with_error(e.to_string(), StatusCode::BadRequest).await,
        },
    }
}
async fn send_to_writer(
    write_request: Option<WasmModule>,
    id: String,
    write_type: WriteOps,
    tx: Sender<WriteJob>,
) -> Result<Response, Error> {
    let write_job = WriteJob::new(write_request, write_type, id.clone());

    task::spawn(async move {
        if let Err(e) = tx.send(write_job).await {
            log_error!(RequestError::ChannelError(e.to_string()));
        }
        WRITER_CHANNEL_QUEUE.set(tx.capacity().try_into().unwrap());
    });

    respond(serde_json::json!({
        "id": id
    }))
    .await
}

async fn verify_id(
    req: &Request<AppState>,
    reader_tx: Sender<ReadJob>,
) -> Result<String, RequestError> {
    let id = req.param("id").unwrap();
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();
    let read_job = ReadJob::new(done_tx, id.to_string());

    reader_tx.send(read_job).await.unwrap();

    match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Success(_) => Ok(id.to_string()),
            ReadResponse::Fail(e) => Err(RequestError::InvalidId(e)),
        },
        Err(e) => Err(log_error!(RequestError::ChannelError(e.to_string()))),
    }
}

async fn deserialize_request(req: &mut Request<AppState>) -> Result<WasmModule, RequestError> {
    req.body_json::<WasmModule>()
        .await
        .map_err(|e| log_error!(RequestError::InvalidJson(e.to_string())))
}

fn random_id() -> String {
    let mut rng = rand::thread_rng();
    let random_number: u32 = rng.gen_range(0..u32::MAX);

    digest(random_number.to_be_bytes().as_slice())
}
