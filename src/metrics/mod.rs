pub mod constants;

use self::constants::{CPU_USAGE, DATABASE_SIZE, MEMORY_USAGE};
use crate::database::DB;
use std::{sync::Arc, time::Duration};
use sysinfo::{Process, System};

pub async fn collect_usage_metrics() {
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();
        let process = sys.process(sysinfo::get_current_pid().unwrap()).unwrap();
        let cpu_usage = process.cpu_usage() as i64;
        let memory_usage = process.memory() as i64;
        let db = Arc::clone(&DB);
        let db_size = db
            .lock()
            .unwrap()
            .property_int_value("rocksdb.estimate-live-data-size")
            .unwrap()
            .unwrap() as i64;

        CPU_USAGE.set(cpu_usage);
        MEMORY_USAGE.set(memory_usage);
        DATABASE_SIZE.set(db_size);
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
}
