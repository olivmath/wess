use crate::{
    database::models::{TypeArg, WasmModule},
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
    let id = req.param("id").unwrap().to_string();
    let wasm_module = match retrieve_wasm_module(&id, &req).await {
        Ok(wm) => wm,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    let request_args = match deserialize_request(&wasm_module, &mut req).await {
        Ok(args) => args,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    let result = match send_to_runner(id, request_args, req.state().runner_tx.clone()).await {
        Ok(r) => r,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    let response = match serialize_wasm_return(result, &wasm_module.metadata.return_type).await {
        Ok(r) => r,
        Err(e) => return respond_with_error(e.err.to_string(), e.status).await,
    };

    respond(response).await
}

// Função para converter um wasmer::Value em serde_json::Value
fn serialize_wasm_value(value: &wasmer::Value) -> serde_json::Value {
    match value.ty() {
        wasmer::Type::I32 => {
            serde_json::Value::Number(serde_json::Number::from(value.unwrap_i32()))
        }
        wasmer::Type::I64 => {
            serde_json::Value::Number(serde_json::Number::from(value.unwrap_i64()))
        }
        wasmer::Type::F32 => serde_json::Value::Number(
            serde_json::Number::from_f64(f64::from(value.unwrap_f32())).unwrap(),
        ),
        wasmer::Type::F64 => {
            serde_json::Value::Number(serde_json::Number::from_f64(value.unwrap_f64()).unwrap())
        }
        _ => serde_json::Value::Null,
    }
}

async fn serialize_wasm_return(
    result: Box<[wasmer::Value]>,
    return_type: &Vec<Option<TypeArg>>,
) -> Result<Vec<serde_json::Value>, CustomError> {
    let mut serialized_result = Vec::new();

    for (value, expected_type) in result.iter().zip(return_type.iter()) {
        let json_value = match expected_type {
            Some(TypeArg::I32) | Some(TypeArg::I64) | Some(TypeArg::F32) | Some(TypeArg::F64) => {
                serialize_wasm_value(value)
            }
            None => serde_json::Value::Null,
        };

        serialized_result.push(json_value);
    }

    Ok(serialized_result)
}
async fn retrieve_wasm_module(
    id: &String,
    req: &Request<AppState>,
) -> Result<WasmModule, CustomError> {
    let reader_tx = req.state().reader_tx.clone();
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();

    reader_tx
        .send(ReadJob::new(done_tx, id.clone()))
        .await
        .unwrap();

    let x = match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Success(wm) => Ok(wm),
            ReadResponse::Fail(e) => Err(CustomError {
                err: logger::log_error(RequestError::InvalidId(e).to_string()),
                status: StatusCode::NotFound,
            }),
        },
        Err(e) => Err(CustomError {
            err: logger::log_error(RequestError::ChannelError(e.to_string())).to_string(),
            status: StatusCode::InternalServerError,
        }),
    };
    x
}

async fn deserialize_request(
    wasm_module: &WasmModule,
    req: &mut Request<AppState>,
) -> Result<Vec<wasmer::Value>, CustomError> {
    match req.body_json::<Vec<Option<serde_json::Value>>>().await {
        Ok(args) => {
            let arg_types = wasm_module
                .clone()
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
                                return Err(CustomError {
                                    err: logger::log_error(
                                        RequestError::InvalidType(TypeArg::I32).to_string(),
                                    ),
                                    status: StatusCode::BadRequest,
                                });
                            }
                        }
                        TypeArg::I64 => {
                            if let Some(i64_val) = arg_value.as_i64() {
                                dynamic_args.push(wasmer::Value::I64(i64_val));
                            } else {
                                return Err(CustomError {
                                    err: logger::log_error(
                                        RequestError::InvalidType(TypeArg::I64).to_string(),
                                    ),
                                    status: StatusCode::BadRequest,
                                });
                            }
                        }
                        TypeArg::F32 => {
                            if let Some(f32_val) = arg_value.as_f64() {
                                dynamic_args.push(wasmer::Value::F32(f32_val as f32));
                            } else {
                                return Err(CustomError {
                                    err: logger::log_error(
                                        RequestError::InvalidType(TypeArg::F32).to_string(),
                                    ),
                                    status: StatusCode::BadRequest,
                                });
                            }
                        }
                        TypeArg::F64 => {
                            if let Some(f64_val) = arg_value.as_f64() {
                                dynamic_args.push(wasmer::Value::F64(f64_val));
                            } else {
                                return Err(CustomError {
                                    err: logger::log_error(
                                        RequestError::InvalidType(TypeArg::F64).to_string(),
                                    ),
                                    status: StatusCode::BadRequest,
                                });
                            }
                        }
                    }
                }
                Ok(dynamic_args)
            } else {
                Err(CustomError {
                    err: logger::log_error(RequestError::LengthArgsError {
                        expect: arg_types.len(),
                        found: arg_values.len(),
                    })
                    .to_string(),
                    status: StatusCode::BadRequest,
                })
            }
        }
        Err(e) => Err(CustomError {
            err: logger::log_error(RequestError::InvalidJson(e.to_string())).to_string(),
            status: StatusCode::BadRequest,
        }),
    }
}

pub async fn send_to_runner(
    id: String,
    args: Vec<wasmer::Value>,
    runner_tx: Sender<RunJob>,
) -> Result<Box<[wasmer::Value]>, CustomError> {
    let (done_tx, done_rx) = oneshot::channel::<RunResponse>();
    let run_job = RunJob::new(done_tx, args, id);

    runner_tx
        .send(run_job)
        .await
        .map_err(|e| CustomError {
            err: e.to_string(),
            status: StatusCode::InternalServerError,
        })
        .unwrap();

    match done_rx.await {
        Ok(RunResponse::Success(r)) => Ok(r),
        Ok(RunResponse::Fail(f)) => Err(CustomError {
            err: f.to_string(),
            status: StatusCode::InternalServerError,
        }),
        Err(e) => Err(CustomError {
            err: logger::log_error(RequestError::ChannelError(e.to_string()).to_string()),
            status: StatusCode::InternalServerError,
        }),
    }
}

#[derive(Debug)]
pub struct CustomError {
    err: String,
    status: StatusCode,
}
