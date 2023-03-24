pub mod job;

use self::job::Job;
use crate::{database::RocksDB, wasm::JobType};
use tokio::sync::mpsc::Receiver;

pub struct Runner {
    rx: Receiver<Job>,
    db: RocksDB,
}

impl Runner {
    pub fn new(rx: Receiver<Job>, db: RocksDB) -> Self {
        Runner { rx, db }
    }

    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            let wasm_job = job.wasm_job;
            let responder = job.responder;
            let id = wasm_job.id();
            match wasm_job.job_type {
                JobType::Save => {
                    match self.db.put(id.as_str(), wasm_job.to_wasm()) {
                        Ok(id) => {
                            tokio::spawn(async move { responder.send(id) });
                        }
                        Err(e) => {
                            tokio::spawn(async move { responder.send(e.to_string()) });
                        }
                    };
                }
                JobType::Modity => {
                    match self.db.put(&wasm_job.id(), wasm_job.to_wasm()) {
                        Ok(id) => {
                            tokio::spawn(async move { responder.send(id) });
                        }
                        Err(e) => {
                            tokio::spawn(async move { responder.send(e.to_string()) });
                        }
                    };
                }
                JobType::Delete => {
                    match self.db.del(&wasm_job.id()) {
                        Ok(id) => {
                            tokio::spawn(async move { responder.send(id) });
                        }
                        Err(e) => {
                            tokio::spawn(async move { responder.send(e.to_string()) });
                        }
                    };
                }
                JobType::Run => {
                    println!("Run {}()", wasm_job.to_wasm().metadata.fn_main);
                }
            }
        }
    }
}
