pub mod models;

use self::models::{RJob, ReadResponse};
use crate::database::RocksDB;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct Reader {
    rx: Receiver<RJob>,
    db: RocksDB,
}

impl Reader {
    pub fn new(db: RocksDB) -> (Sender<RJob>, Arc<Mutex<Reader>>) {
        let (tx, rx) = mpsc::channel::<RJob>(100);
        (tx, Arc::new(Mutex::new(Reader { rx, db })))
    }

    pub async fn run(&mut self) {
        while let Some(job) = self.rx.recv().await {
            //
            let responder = job.responder;
            let id = job.id;
            //
            match self.db.get(id.as_str()) {
                Some(wasm_fn) => {
                    tokio::spawn(async move { responder.send(ReadResponse::new(wasm_fn)) });
                }
                None => {
                    tokio::spawn(async move {
                        responder.send(ReadResponse::fail("wasm fn not found".into()))
                    });
                }
            };
        }
    }
}
