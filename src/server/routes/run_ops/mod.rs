use self::utils::{
    deserialize_request, get_id_from_request, retrieve_wasm_module, send_to_runner,
    serialize_wasm_return,
};
use crate::server::{
    response::{respond, respond_with_error},
    AppState,
};
use tide::{Error, Request, Response};

mod utils;

pub async fn make_run_op(mut req: Request<AppState>) -> Result<Response, Error> {
    let id = match get_id_from_request(&req) {
        Ok(id) => id,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    let wasm_module = match retrieve_wasm_module(&id, &req).await {
        Ok(wm) => wm,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    let request_args = match deserialize_request(&wasm_module, &mut req).await {
        Ok(args) => args,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    let result = match send_to_runner(id.clone(), request_args, req.state().runner_tx.clone()).await
    {
        Ok(r) => r,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    let response = match serialize_wasm_return(result, &wasm_module.metadata.return_type).await {
        Ok(r) => r,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    respond(response).await
}
