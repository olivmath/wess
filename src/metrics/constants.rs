use lazy_static::lazy_static;
use prometheus::{
    exponential_buckets, register_histogram, register_histogram_vec, register_int_counter,
    register_int_counter_vec, register_int_gauge, Histogram, HistogramVec, IntCounter,
    IntCounterVec, IntGauge,
};

#[rustfmt::skip]
lazy_static! {
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "http_requests_total",
        "Total number of HTTP requests",
        &["path"]
    ).unwrap_or_else(|e| panic!("Metric create `HTTP_REQUESTS_TOTAL` failed,  {}", e));

    pub static ref CONCURRENT_CONNECTIONS: IntGauge = register_int_gauge!(
        "concurrent_connections",
        "Number of concurrent connections"
    ).unwrap_or_else(|e| panic!("Metric create `CONCURRENT_CONNECTIONS` failed,  {}", e));


    pub static ref HTTP_REQUEST_LATENCY: Histogram = register_histogram!(
        "http_request_latency_seconds",
        "HTTP request latency in seconds",
        exponential_buckets(0.01, 2.0, 10).unwrap_or_else(|e| panic!("Metric create `HTTP_REQUEST_LATENCY` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `HTTP_REQUEST_LATENCY` failed,  {}", e));

    pub static ref ERROR_COUNT: IntCounter = register_int_counter!(
        "error_count_total",
        "Total number of errors"
    ).unwrap_or_else(|e| panic!("Metric create `ERROR_COUNT` failed,  {}", e));

    pub static ref DATABASE_OPERATIONS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "database_operations_total",
        "Total number of database operations",
        &["operation"]
    ).unwrap_or_else(|e| panic!("Metric create `DATABASE_OPERATIONS_TOTAL` failed,  {}", e));

    pub static ref DATABASE_OPERATION_DURATION: HistogramVec = register_histogram_vec!(
        "database_operation_duration_seconds",
        "Duration of database operations in seconds",
        &["operation"],
        exponential_buckets(0.01, 2.0, 10).unwrap_or_else(|e| panic!("Metric create `DATABASE_OPERATION_DURATION` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `DATABASE_OPERATION_DURATION` failed,  {}", e));

    pub static ref CPU_USAGE: IntGauge = register_int_gauge!(
        "cpu_usage",
        "CPU usage in percentage"
    ).unwrap_or_else(|e| panic!("Metric create ` CPU_USAGE` failed,  {}", e));

    pub static ref MEMORY_USAGE: IntGauge = register_int_gauge!(
        "memory_usage",
        "Memory usage in bytes"
    ).unwrap_or_else(|e| panic!("Metric create `MEMORY_USAGE` failed,  {}", e));
    
    pub static ref DATABASE_SIZE: IntGauge = register_int_gauge!(
        "database_size_bytes",
        "Size of the database in bytes"
    ).unwrap_or_else(|e| panic!("Metric create `DATABASE_SIZE` failed,  {}", e));

    pub static ref WASM_COMPILER_TIME: HistogramVec = register_histogram_vec!(
        "wasm_compiler_time_seconds",
        "Compiler Wasm time in nanoseconds",
        &["wasm_module"],
        exponential_buckets(1.0, 2.0, 10).unwrap_or_else(|e| panic!("Metric create `WASM_COMPILER_TIME` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `WASM_COMPILER_TIME` failed,  {}", e));
    
    pub static ref WASM_EXECUTION_TIME: HistogramVec = register_histogram_vec!(
        "wasm_execution_time_seconds",
        "Execution Wasm function time in seconds",
        &["function_name"],
        exponential_buckets(1.0, 2.0, 10).unwrap_or_else(|e| panic!("Metric create `WASM_EXECUTION_TIME` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `WASM_EXECUTION_TIME` failed, {}", e));
}
