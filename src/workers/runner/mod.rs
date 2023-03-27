pub mod models;

use self::models::{RunJob, RunResponse, RunnerError};
use crate::{
    database::{models::WasmFn, RocksDB},
    server::models::RunRequest,
};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct Runner {
    rx: Receiver<RunJob>,
    db: RocksDB,
}

impl Runner {
    pub fn new(db: RocksDB) -> (Sender<RunJob>, Arc<Mutex<Runner>>) {
        let (tx, rx) = mpsc::channel::<RunJob>(100);
        (tx, Arc::new(Mutex::new(Runner { rx, db })))
    }

    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let responder = job.responder;
            let args = job.args;
            let id = job.id;
            //
            match self.db.get(id.as_str()) {
                Some(wasm_fn) => match self.run_function(&wasm_fn, &args).await {
                    Ok(result) => {
                        tokio::spawn(async move { responder.send(result) });
                    }
                    Err(e) => {
                        tokio::spawn(async move { responder.send(RunResponse::Fail(e)) });
                    }
                },
                None => {
                    tokio::spawn(async move {
                        responder.send(RunResponse::fail(RunnerError::WasmNotFound))
                    });
                }
            };
        }
    }

    pub async fn run_function(
        &mut self,
        _wasm_fn: &WasmFn,
        _args: &RunRequest,
    ) -> Result<RunResponse, RunnerError> {
        //
        Ok(RunResponse::new("foi".into()))
    }
}
