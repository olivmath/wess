use rand::Rng;
use serde_json::json;
use sha256::digest;
use tide::{Error, Request, Response, StatusCode};
use tokio::sync::{mpsc::Sender, oneshot};

use crate::{
    runner::job::Job,
    wasm::{JobType, WasmJob},
};

pub async fn get_wasm(req: Request<AppState>) -> Result<Response, Error> {
    let rocksdb = RocksDB::new();

    if let Some(wasm) = rocksdb.get(req.param("id").unwrap()) {
        Ok(wasm.into())
    } else {
        Ok(Response::builder(StatusCode::NotFound)
            .body("Invalid ID")
            .build())
    }
}

async fn send_to_runner(wasm_job: WasmJob, tx: Sender<Job>) -> Result<Response, Error> {
    let (done_tx, done_rx) = oneshot::channel::<String>();

    tx.send(Job {
        wasm_job: wasm_job.clone(),
        responder: done_tx,
    })
    .await
    .unwrap();

    match done_rx.await {
        Ok(response) => Ok(Response::builder(StatusCode::Created)
            .body(json!({ "message": response }))
            .build()),
        Err(e) => Ok(Response::builder(StatusCode::InternalServerError)
            .body(json!({ "message": e.to_string() }))
            .build()),
    }
}

pub async fn job_maker(mut req: Request<AppState>, job_type: JobType) -> Result<Response, Error> {
    if let Ok(wasm_req) = req.body_json::<WasmRequest>().await {
        send_to_runner(
            WasmJob {
                job_type,
                id: "0x22ff".to_string(), // TODO hash the .wasm with SHA256
                wasm_req,
            },
            req.state().tx.clone(),
        )
        .await
    } else {
        Ok(Response::builder(StatusCode::NotFound)
            .body(json!({"message": "invalid json"}))
            .build())
    }
}
