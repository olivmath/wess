use tide::{Error, Request, Response, StatusCode};


pub fn get_wasm(_req: Request<()>) -> Result<Response, Error> {
    Ok(Response::builder(StatusCode::Ok).body("get wasm").build())
}

async fn send_to_runner(wasm_job: WasmJob, tx: Sender<WasmJob>) -> Result<Response, Error> {
    tx.send(wasm_job.clone()).await.unwrap();

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
