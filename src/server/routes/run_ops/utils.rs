use crate::{
    database::models::WasmModule,
    errors::WessError,
    metrics::constants::RUNNER_CHANNEL_QUEUE,
    server::AppState,
    workers::{
        reader::models::{ReadJob, ReadResponse},
        runner::models::{RunJob, RunResponse},
    },
};
use tide::Request;
use tokio::sync::{mpsc::Sender, oneshot};

pub async fn serialize_wasm_return(
    result: Box<[wasmer::Value]>,
    return_type: &Vec<Option<wasmer::Type>>,
) -> Result<Vec<serde_json::Value>, WessError> {
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
) -> Result<WasmModule, WessError> {
    let reader_tx = req.state().reader_tx.clone();
    let (done_tx, done_rx) = oneshot::channel::<ReadResponse>();

    reader_tx
        .send(ReadJob::new(done_tx, Some(id.clone())))
        .await
        .unwrap();

    match done_rx.await {
        Ok(response) => match response {
            ReadResponse::Module(wm) => Ok(wm),
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

pub async fn deserialize_request(
    wasm_module: &WasmModule,
    req: &mut Request<AppState>,
) -> Result<Vec<wasmer::Value>, WessError> {
    let (arg_types, arg_values) = parse_request_args(wasm_module, req).await?;

    let dynamic_args: Result<Vec<wasmer::Value>, WessError> = arg_types
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
) -> Result<Box<[wasmer::Value]>, WessError> {
    let (done_tx, done_rx) = oneshot::channel::<RunResponse>();
    let run_job = RunJob::new(done_tx, args, id);

    runner_tx
        .send(run_job)
        .await
        .map_err(|e| log_error!(e.to_string(), 500))
        .unwrap();
    RUNNER_CHANNEL_QUEUE.set(runner_tx.capacity() as i64);

    match done_rx.await {
        Ok(RunResponse::Success(r)) => Ok(r),
        Ok(RunResponse::Fail(f)) => {
            let werr = log_error!(f.to_string(), 500);
            Err(werr)
        }
        Err(e) => {
            let werr = log_error!(format!("Channel Error: {}", e.to_string()), 500);
            Err(werr)
        }
    }
}

pub fn get_id_from_request(req: &Request<AppState>) -> Result<String, WessError> {
    req.param("id")
        .map(|id| id.to_string())
        .map_err(|e| log_error!(e.to_string(), 400))
}

async fn parse_request_args(
    wasm_module: &WasmModule,
    req: &mut Request<AppState>,
) -> Result<(Vec<wasmer::Type>, Vec<serde_json::Value>), WessError> {
    let args: Vec<Option<serde_json::Value>> = req
        .body_json()
        .await
        .map_err(|e| log_error!(format!("Invalid Json: {}", e.to_string()), 400))?;

    let arg_types = wasm_module
        .metadata
        .args
        .iter()
        .filter_map(|t| t.clone())
        .collect::<Vec<wasmer::Type>>();

    let arg_values: Vec<serde_json::Value> = args.iter().filter_map(|v| v.clone()).collect();

    if arg_types.len() != arg_values.len() {
        let werr = log_error!(
            format!(
                "Length Args Error: expect: {}, found: {}",
                arg_types.len(),
                arg_values.len()
            ),
            400
        );
        return Err(werr);
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
) -> Result<wasmer::Value, WessError> {
    match arg_type {
        wasmer::Type::I32 => arg_value
            .as_i64()
            .map(|i| wasmer::Value::I32(i as i32))
            .ok_or_else(|| log_error!(format!("Invalid Type: I32"), 400)),

        wasmer::Type::I64 => arg_value
            .as_i64()
            .map(|i| wasmer::Value::I64(i))
            .ok_or_else(|| log_error!(format!("Invalid Type: I64"), 400)),

        wasmer::Type::F32 => arg_value
            .as_f64()
            .map(|f| wasmer::Value::F32(f as f32))
            .ok_or_else(|| log_error!(format!("Invalid Type: F32"), 400)),

        wasmer::Type::F64 => arg_value
            .as_f64()
            .map(|f| wasmer::Value::F64(f))
            .ok_or_else(|| log_error!(format!("Invalid Type: F64"), 400)),

        _ => Err(log_error!(format!("Invalid Type: {}", *arg_type), 400)),
    }
}
