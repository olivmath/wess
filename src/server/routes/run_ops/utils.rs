use crate::{
    database::models::WasmModule,
    metrics::constants::RUNNER_CHANNEL_QUEUE,
    server::{errors::RequestError, AppState},
    workers::{
        reader::models::{ReadJob, ReadResponse},
        runner::models::{RunJob, RunResponse},
    },
};
use tide::{Request, StatusCode};
use tokio::sync::{mpsc::Sender, oneshot};

#[derive(Debug)]
pub struct CustomError {
    pub err: String,
    pub status: StatusCode,
}

pub async fn serialize_wasm_return(
    result: Box<[wasmer::Value]>,
    return_type: &Vec<Option<wasmer::Type>>,
) -> Result<Vec<serde_json::Value>, CustomError> {
    let serialized_result = result
        .iter()
        .zip(return_type.iter())
        .map(|(value, expected_type)| match expected_type {
            Some(wasmer::Type::I32)
            | Some(wasmer::Type::I64)
            | Some(wasmer::Type::F32)
            | Some(wasmer::Type::F64) => serialize_wasm_value(value),
            Some(wasmer::Type::V128)
            | Some(wasmer::Type::ExternRef)
            | Some(wasmer::Type::FuncRef) => serde_json::Value::Null,
            None => serde_json::Value::Null,
        })
        .collect();

    Ok(serialized_result)
}

pub async fn retrieve_wasm_module(
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
                err: log_error!(RequestError::InvalidId(e).to_string()),
                status: StatusCode::NotFound,
            }),
        },
        Err(e) => Err(CustomError {
            err: log_error!(RequestError::ChannelError(e.to_string())).to_string(),
            status: StatusCode::InternalServerError,
        }),
    };
    x
}

pub async fn deserialize_request(
    wasm_module: &WasmModule,
    req: &mut Request<AppState>,
) -> Result<Vec<wasmer::Value>, CustomError> {
    let (arg_types, arg_values) = parse_request_args(wasm_module, req).await?;

    let dynamic_args: Result<Vec<wasmer::Value>, CustomError> = arg_types
        .iter()
        .zip(arg_values.iter())
        .map(|(arg_type, arg_value)| map_json_value_to_wasmer_value(arg_type, arg_value))
        .collect();

    dynamic_args
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
    RUNNER_CHANNEL_QUEUE.set(runner_tx.capacity() as i64);

    match done_rx.await {
        Ok(RunResponse::Success(r)) => Ok(r),
        Ok(RunResponse::Fail(f)) => Err(CustomError {
            err: f.to_string(),
            status: StatusCode::InternalServerError,
        }),
        Err(e) => Err(CustomError {
            err: log_error!(RequestError::ChannelError(e.to_string()).to_string()),
            status: StatusCode::InternalServerError,
        }),
    }
}

pub fn get_id_from_request(req: &Request<AppState>) -> Result<String, CustomError> {
    req.param("id")
        .map(|id| id.to_string())
        .map_err(|e| CustomError {
            err: e.to_string(),
            status: StatusCode::BadRequest,
        })
}

async fn parse_request_args(
    wasm_module: &WasmModule,
    req: &mut Request<AppState>,
) -> Result<(Vec<wasmer::Type>, Vec<serde_json::Value>), CustomError> {
    let args: Vec<Option<serde_json::Value>> = req.body_json().await.map_err(|e| CustomError {
        err: log_error!(RequestError::InvalidJson(e.to_string())).to_string(),
        status: StatusCode::BadRequest,
    })?;

    let arg_types = wasm_module
        .metadata
        .args
        .iter()
        .filter_map(|t| t.clone())
        .collect::<Vec<wasmer::Type>>();

    let arg_values: Vec<serde_json::Value> = args.iter().filter_map(|v| v.clone()).collect();

    if arg_types.len() != arg_values.len() {
        return Err(CustomError {
            err: log_error!(RequestError::LengthArgsError {
                expect: arg_types.len(),
                found: arg_values.len(),
            })
            .to_string(),
            status: StatusCode::BadRequest,
        });
    }

    Ok((arg_types, arg_values))
}

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

fn map_json_value_to_wasmer_value(
    arg_type: &wasmer::Type,
    arg_value: &serde_json::Value,
) -> Result<wasmer::Value, CustomError> {
    match arg_type {
        wasmer::Type::I32 => arg_value
            .as_i64()
            .map(|i| wasmer::Value::I32(i as i32))
            .ok_or_else(|| CustomError {
                err: log_error!(RequestError::InvalidType(wasmer::Type::I32).to_string()),
                status: StatusCode::BadRequest,
            }),

        wasmer::Type::I64 => arg_value
            .as_i64()
            .map(|i| wasmer::Value::I64(i))
            .ok_or_else(|| CustomError {
                err: log_error!(RequestError::InvalidType(wasmer::Type::I64).to_string()),
                status: StatusCode::BadRequest,
            }),

        wasmer::Type::F32 => arg_value
            .as_f64()
            .map(|f| wasmer::Value::F32(f as f32))
            .ok_or_else(|| CustomError {
                err: log_error!(RequestError::InvalidType(wasmer::Type::F32).to_string()),
                status: StatusCode::BadRequest,
            }),

        wasmer::Type::F64 => arg_value
            .as_f64()
            .map(|f| wasmer::Value::F64(f))
            .ok_or_else(|| CustomError {
                err: log_error!(RequestError::InvalidType(wasmer::Type::F64).to_string()),
                status: StatusCode::BadRequest,
            }),

        _ => Err(CustomError {
            err: log_error!(RequestError::InvalidType(*arg_type)).to_string(),
            status: StatusCode::BadRequest,
        }),
    }
}
