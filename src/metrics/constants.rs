use lazy_static::lazy_static;
use prometheus::{
    register_counter, register_histogram, register_int_counter, register_int_gauge, Counter,
    Histogram, IntCounter, IntGauge,
};

#[rustfmt::skip]
lazy_static! {
    pub static ref CONCURRENT_CONNECTIONS: IntGauge = register_int_gauge!(
        "wess_concurrent_connections",
        "Number of concurrent connections"
    ).unwrap();

    pub static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "wess_http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();

    pub static ref HTTP_REQUEST_LATENCY: Histogram = register_histogram!(
        "wess_http_request_latency_seconds",
        "HTTP request latency in seconds"
    ).unwrap();

    pub static ref ERROR_COUNT: IntCounter = register_int_counter!(
        "wess_error_count_total",
        "Total number of errors"
    ).unwrap();

    pub static ref CPU_USAGE: IntGauge = register_int_gauge!(
        "wess_cpu_usage",
        "CPU usage in percentage"
    ).unwrap();

    pub static ref MEMORY_USAGE: IntGauge = register_int_gauge!(
        "wess_memory_usage",
        "Memory usage in bytes"
    ).unwrap();

    pub static ref DATABASE_OPERATIONS_TOTAL: IntCounter = register_int_counter!(
        "wess_database_operations_total",
        "Total number of database operations"
    ).unwrap();

    pub static ref DATABASE_OPERATION_DURATION: Histogram = register_histogram!(
        "wess_database_operation_duration_seconds",
        "Duration of database operations in seconds"
    ).unwrap();

    pub static ref DATABASE_SIZE: IntGauge = register_int_gauge!(
        "wess_database_size_bytes",
        "Size of the database in bytes"
    ).unwrap();

    pub static ref WASM_COMPILER_TIME: Histogram = register_histogram!(
        "wess_wasm_compiler_time_seconds",
        "Compiler Wasm time in seconds"
    ).unwrap();

    pub static ref WASM_EXECUTION_TIME: Histogram = register_histogram!(
        "wess_wasm_execution_time_seconds",
        "Execution Wasm function time in seconds"
    ).unwrap();
}
