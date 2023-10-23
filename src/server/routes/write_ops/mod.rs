use std::convert::TryInto;

use crate::{
    database::models::WasmModule,
    errors::WessError,
    metrics::constants::WRITER_CHANNEL_QUEUE,
    server::{
        response::{respond, respond_with_error},
        AppState,
    },
    workers::{
        reader::models::{ReadJob, ReadResponse},
        writer::models::{WriteJob, WriteOps},
    },
};
use async_std::task;
use tide::{Error, Request, Response};
use tokio::sync::{mpsc::Sender, oneshot};
use uuid::Uuid;
use wasmer::{Engine, Module};

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
            let id: Uuid = Uuid::new_v4();
            match deserialize_request(&mut req).await {
                Ok(body_request) => {
                    send_to_writer(
                        Some(body_request),
                        id.to_string(),
                        req.state().writer_tx.clone(),
                    )
                    .await
                }
                Err(e) => respond_with_error(e).await,
            }
        }
        WriteOps::Update => match verify_id(&req, reader_tx).await {
            Ok(id) => match deserialize_request(&mut req).await {
                Ok(write_request) => {
                    send_to_writer(Some(write_request), id, req.state().writer_tx.clone()).await
                }
                Err(e) => respond_with_error(e).await,
            },
            Err(e) => respond_with_error(e).await,
        },
        WriteOps::Delete => match verify_id(&req, reader_tx).await {
            Ok(id) => send_to_writer(None, id, req.state().writer_tx.clone()).await,
            Err(e) => respond_with_error(e).await,
        },
    }
}

async fn send_to_writer(
    write_request: Option<WasmModule>,
    id: String,
    tx: Sender<WriteJob>,
) -> Result<Response, Error> {
    let write_job = WriteJob::new(write_request, id.clone());

    task::spawn(async move {
        if let Err(e) = tx.send(write_job).await {
            log_error!(format!("Channel Error: {}", e.to_string()), 500);
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
) -> Result<String, WessError> {
    let id = req.param("id").unwrap();
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();
    let read_job = ReadJob::new(done_tx, Some(id.to_string()));

    reader_tx.send(read_job).await.unwrap();

    match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Module(_) => Ok(id.to_string()),
            ReadResponse::Fail(e) => {
                let werr = log_error!(format!("Invalid Id: {}", e.to_string()), 404);
                Err(werr)
            }
            _ => unreachable!(),
        },
        Err(e) => {
            let werr = log_error!(format!("Channel Error: {}", e.to_string()), 500);
            Err(werr)
        }
    }
}

async fn deserialize_request(req: &mut Request<AppState>) -> Result<WasmModule, WessError> {
    req.body_json::<WasmModule>()
        .await
        .map_err(|e| log_error!(format!("Inavlid Json: {}", e), 400))
        .and_then(|wm| match Module::validate(&Engine::default(), &wm.wasm) {
            Ok(_) => Ok(wm),
            Err(e) => {
                let werr = log_error!(format!("Invalid Wasm: {}", e), 400);
                Err(werr)
            }
        })
}
