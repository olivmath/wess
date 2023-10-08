use crate::{
    database::models::TypeArg,
    logger,
    server::{
        errors::RequestError,
        response::{respond, respond_with_error},
        AppState,
    },
    workers::{
        reader::models::{ReadJob, ReadResponse},
        runner::models::{RunJob, RunResponse},
    },
};

use tide::{Error, Request, Response, StatusCode};
use tokio::sync::{mpsc::Sender, oneshot};

/// # Handler function for run operations.
///
/// ## Arguments
///
/// * `req` - The [`Request`] object containing the run operation to perform.
///
/// ## Returns
///
/// A [`Result`] containing the [`Response`] object.
pub async fn make_run_op(mut req: Request<AppState>) -> Result<Response, Error> {
    let reader_tx = req.state().reader_tx.clone();
    let id = req.param("id").unwrap().to_string();
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();

    reader_tx
        .send(ReadJob::new(done_tx, id.clone()))
        .await
        .unwrap();

    let wasm_module = match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Success(wm) => wm,
            ReadResponse::Fail(e) => {
                return respond_with_error(
                    logger::log_error(RequestError::InvalidId(e).to_string()),
                    StatusCode::NotFound,
                )
                .await
            }
        },
        Err(e) => {
            return respond_with_error(
                logger::log_error(RequestError::ChannelError(e.to_string()).to_string()),
                StatusCode::InternalServerError,
            )
            .await
        }
    };

    match req.body_json::<Vec<Option<serde_json::Value>>>().await {
        Ok(args) => {
            let arg_types = wasm_module
                .metadata
                .args
                .into_iter()
                .filter_map(|t| t)
                .collect::<Vec<TypeArg>>();
            let arg_values = args
                .into_iter()
                .filter_map(|v| v)
                .collect::<Vec<serde_json::Value>>();
            let mut dynamic_args = Vec::new();

            if arg_types.len() == arg_values.len() {
                for (arg_type, arg_value) in arg_types.iter().zip(arg_values.iter()) {
                    match arg_type {
                        TypeArg::I32 => {
                            if let Some(i32_val) = arg_value.as_i64() {
                                dynamic_args.push(wasmer::Value::I32(i32_val as i32));
                            } else {
                                let e = logger::log_error(RequestError::InvalidJson(
                                    "Invalide value to i32".to_string(),
                                ));
                                return respond_with_error(e.to_string(), StatusCode::BadRequest)
                                    .await;
                            }
                        }
                        TypeArg::I64 => {
                            if let Some(i64_val) = arg_value.as_i64() {
                                dynamic_args.push(wasmer::Value::I64(i64_val));
                            } else {
                                let e = logger::log_error(RequestError::InvalidJson(
                                    "Invalide value to i64".to_string(),
                                ));
                                return respond_with_error(e.to_string(), StatusCode::BadRequest)
                                    .await;
                            }
                        }
                        TypeArg::F32 => {
                            if let Some(f32_val) = arg_value.as_f64() {
                                dynamic_args.push(wasmer::Value::F32(f32_val as f32));
                            } else {
                                let e = logger::log_error(RequestError::InvalidJson(
                                    "Invalide value to f32".to_string(),
                                ));
                                return respond_with_error(e.to_string(), StatusCode::BadRequest)
                                    .await;
                            }
                        }
                        TypeArg::F64 => {
                            if let Some(f64_val) = arg_value.as_f64() {
                                dynamic_args.push(wasmer::Value::F64(f64_val));
                            } else {
                                let e = logger::log_error(RequestError::InvalidJson(
                                    "Invalide value to f64".to_string(),
                                ));
                                return respond_with_error(e.to_string(), StatusCode::BadRequest)
                                    .await;
                            }
                        }
                    }
                }

                send_to_runner(id, dynamic_args, req.state().runner_tx.clone()).await
            } else {
                let e = logger::log_error(RequestError::InvalidJson(format!(
                    "Wrong length of args, expect {}, passed {}",
                    arg_types.len(),
                    arg_values.len()
                )));
                respond_with_error(e.to_string(), StatusCode::BadRequest).await
            }
        }
        Err(e) => {
            let e = logger::log_error(RequestError::InvalidJson(e.to_string()));
            respond_with_error(e.to_string(), StatusCode::BadRequest).await
        }
    }
}

/// # Sends a [`RunJob`] message to the `runner` worker to execute a WebAssembly function.
///
/// ## Arguments
///
/// * `id` - A [`String`] representing the ID of the WebAssembly function to execute.
/// * `args` - A [`RunRequest`] object containing the arguments to pass to the WebAssembly function.
/// * `runner_tx` - A [`Sender`] for the `runner` worker's channel.
///
/// ## Returns
///
/// * A [`Result`] containing a [`Response`] with a serialized [`RunResponse`] message or an [`Error`]
/// response.
pub async fn send_to_runner(
    id: String,
    args: Vec<wasmer::Value>,
    runner_tx: Sender<RunJob>,
) -> Result<Response, Error> {
    let (done_tx, done_rx) = oneshot::channel::<RunResponse>();
    let run_job = RunJob::new(done_tx, args, id);

    runner_tx.send(run_job).await.unwrap();

    match done_rx.await {
        Ok(response) => match response {
            RunResponse::Success(r) => respond(r).await,
            RunResponse::Fail(f) => {
                respond_with_error(f.to_string(), StatusCode::InternalServerError).await
            }
        },
        Err(e) => {
            logger::log_error(RequestError::ChannelError(e.to_string()));
            respond_with_error(e.to_string(), StatusCode::InternalServerError).await
        }
    }
}
