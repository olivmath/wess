use serde_json::json;
use tide::{Error, Request, Response, StatusCode};
use tokio::sync::mpsc::Sender;

use super::{
    super::database::RocksDB,
    request::{JobType, WasmJob, WasmRequest},
    AppState,
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

    Ok(Response::builder(StatusCode::Created)
        .body(json!({ "id": wasm_job.id }))
        .build())
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
