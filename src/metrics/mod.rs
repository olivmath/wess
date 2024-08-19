pub mod constants;

use self::constants::{CPU_USAGE, DATABASE_SIZE, MEMORY_USAGE, APP_VERSION};
use crate::database::DB;
use std::{sync::Arc, time::Duration};
use sysinfo::{get_current_pid, System};

pub async fn collect_usage_metrics() {
    let version = env!("CARGO_PKG_VERSION");
    APP_VERSION.with_label_values(&[version]).set(1);

    loop {
        let mut sys = System::new();
        sys.refresh_all();

        MEMORY_USAGE.set(sys.used_memory() as i64);
        if let Ok(pid) = get_current_pid() {
            CPU_USAGE.set(sys.process(pid).unwrap().cpu_usage() as i64);
        }

        let db = Arc::clone(&DB);
        let db_size = db
            .lock()
            .unwrap()
            .property_int_value("rocksdb.estimate-live-data-size")
            .unwrap()
            .unwrap() as i64;

        DATABASE_SIZE.set(db_size);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
