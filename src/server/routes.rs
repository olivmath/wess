use tide::{Error, Request, Response, StatusCode};


pub fn get_wasm(_req: Request<()>) -> Result<Response, Error> {
    Ok(Response::builder(StatusCode::Ok).body("get wasm").build())
}

pub fn add_new_wasm(_req: Request<()>) -> Result<Response, Error> {
    Ok(Response::builder(StatusCode::Ok).body("new wasm").build())
}

pub fn update_wasm(_req: Request<()>) -> Result<Response, Error> {
    Ok(Response::builder(StatusCode::Ok)
        .body("update wasm")
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
