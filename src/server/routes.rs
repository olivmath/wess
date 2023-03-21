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

pub fn remove_wasm(_req: Request<()>) -> Result<Response, Error> {
    Ok(Response::builder(StatusCode::Ok)
        .body("delete wasm")
        .build())
}
