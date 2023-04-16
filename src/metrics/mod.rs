pub mod constants;

use crate::database::DB;
use constants::VIRTUAL_MEMORY_USAGE;
use std::{sync::Arc, time::Duration};
use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};
use tokio::time;

use self::constants::DATABASE_SIZE;

pub async fn collect_hardware_metrics() {
    let mut interval = time::interval(Duration::from_secs(5));
    let mut system = System::new_all();
    let db = Arc::clone(&DB);
    let pid = match get_current_pid() {
        Ok(pid) => pid,
        Err(e) => {
            panic!("failed to get current pid: {}", e);
        }
    };

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            system.refresh_all();

            let db_size = db
                .lock()
                .unwrap()
                .property_int_value("rocksdb.estimate-live-data-size")
                .unwrap()
                .unwrap() as i64;
            DATABASE_SIZE.set(db_size);
            if let Some(process) = system.process(pid) {
                VIRTUAL_MEMORY_USAGE.set(process.virtual_memory() as i64);
            }
        }
    });
}
