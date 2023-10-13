use lazy_static::lazy_static;
use prometheus::{
    exponential_buckets, register_histogram, register_histogram_vec, register_int_counter,
    register_int_counter_vec, register_int_gauge, Histogram, HistogramVec, IntCounter,
    IntCounterVec, IntGauge,
};

#[rustfmt::skip]
lazy_static! {
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "wess_http_requests_total",
        "Total number of HTTP requests",
        &["path"]
    ).unwrap_or_else(|e| panic!("Metric create `HTTP_REQUESTS_TOTAL` failed,  {}", e));

    pub static ref CONCURRENT_CONNECTIONS: IntGauge = register_int_gauge!(
        "wess_concurrent_connections",
        "Number of concurrent connections"
    ).unwrap_or_else(|e| panic!("Metric create `CONCURRENT_CONNECTIONS` failed,  {}", e));

    pub static ref READER_CHANNEL_QUEUE: IntGauge = register_int_gauge!(
        "wess_reader_channel_queue",
        "All requests in the Reader channel queue"
    ).unwrap_or_else(|e| panic!("Metric create `READER_CHANNEL_QUEUE` failed,  {}", e));

    pub static ref WRITER_CHANNEL_QUEUE: IntGauge = register_int_gauge!(
        "wess_writer_channel_queue",
        "All requests in the Writer channel queue"
    ).unwrap_or_else(|e| panic!("Metric create `WRITER_CHANNEL_QUEUE` failed,  {}", e));

    pub static ref RUNNER_CHANNEL_QUEUE: IntGauge = register_int_gauge!(
        "wess_runner_channel_queue",
        "All requests in the Runner channel queue"
    ).unwrap_or_else(|e| panic!("Metric create `RUNNER_CHANNEL_QUEUE` failed,  {}", e));

    pub static ref HTTP_REQUEST_LATENCY: Histogram = register_histogram!(
        "wess_http_request_latency_seconds",
        "HTTP request latency in seconds",
        exponential_buckets(0.00005, 2.0, 12).unwrap_or_else(|e| panic!("Metric create `HTTP_REQUEST_LATENCY` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `HTTP_REQUEST_LATENCY` failed,  {}", e));

    pub static ref ERROR_COUNT: IntCounter = register_int_counter!(
        "wess_error_count_total",
        "Total number of errors"
    ).unwrap_or_else(|e| panic!("Metric create `ERROR_COUNT` failed,  {}", e));

    pub static ref DATABASE_OPERATIONS_TOTAL: IntCounterVec = register_int_counter_vec!(
        "wess_database_operations_total",
        "Total number of database operations",
        &["operation"]
    ).unwrap_or_else(|e| panic!("Metric create `DATABASE_OPERATIONS_TOTAL` failed,  {}", e));

    pub static ref DATABASE_OPERATION_DURATION: HistogramVec = register_histogram_vec!(
        "wess_database_operation_duration_seconds",
        "Duration of database operations in seconds",
        &["operation"],
        exponential_buckets(0.000005, 2.0, 12).unwrap_or_else(|e| panic!("Metric create `DATABASE_OPERATION_DURATION` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `DATABASE_OPERATION_DURATION` failed,  {}", e));

    pub static ref CPU_USAGE: IntGauge = register_int_gauge!(
        "wess_cpu_usage",
        "CPU usage in percentage"
    ).unwrap_or_else(|e| panic!("Metric create ` CPU_USAGE` failed,  {}", e));

    pub static ref MEMORY_USAGE: IntGauge = register_int_gauge!(
        "wess_memory_usage",
        "Memory usage in bytes"
    ).unwrap_or_else(|e| panic!("Metric create `MEMORY_USAGE` failed,  {}", e));
    
    pub static ref DATABASE_SIZE: IntGauge = register_int_gauge!(
        "wess_database_size_bytes",
        "Size of the database in bytes"
    ).unwrap_or_else(|e| panic!("Metric create `DATABASE_SIZE` failed,  {}", e));

    pub static ref WASM_COMPILER_TIME: HistogramVec = register_histogram_vec!(
        "wess_wasm_compile_time_seconds",
        "Compile Wasm time in seconds",
        &["wasm_module"],
        exponential_buckets(0.0005, 2.0, 12).unwrap_or_else(|e| panic!("Metric create `WASM_COMPILER_TIME` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `WASM_COMPILER_TIME` failed,  {}", e));
    
    pub static ref WASM_EXECUTION_TIME: HistogramVec = register_histogram_vec!(
        "wess_wasm_execution_time_seconds",
        "Execution Wasm function time in seconds",
        &["function_name"],
        exponential_buckets(0.000005, 2.0, 12).unwrap_or_else(|e| panic!("Metric create `WASM_EXECUTION_TIME` failed, {}", e))
    ).unwrap_or_else(|e| panic!("Metric create `WASM_EXECUTION_TIME` failed, {}", e));
}
