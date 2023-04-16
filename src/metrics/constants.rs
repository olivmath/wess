use lazy_static::lazy_static;
use prometheus::{
    register_counter, register_histogram, register_int_counter,
    register_int_gauge, register_int_gauge_vec, Counter, Histogram, IntCounter,
    IntGauge, IntGaugeVec,
};

#[rustfmt::skip]
lazy_static! {
    pub static ref WESS_BUILD_INFO: IntGaugeVec = register_int_gauge_vec!(
        "wess_build_info",
        "Wess build information",
        &["version"],
    )
    .expect("fail on registering `Wess build information` gauge for prometheus");


    pub static ref WESS_ERRORS: IntCounter = register_int_counter!(
        "wess_errors",
        "Count of errors in the Wess engine",
    )
    .expect("fail on registering `Count of errors in the Wess engine` for prometheus");


    pub static ref WESS_WRITER_REQUEST_QUEUE_SIZE: IntGauge = register_int_gauge!(
        "wess_writer_queue_size",
        "Number of requests at Writer queues"
    )
    .expect("fail on registering `Number of requests at Writer queues` for prometheus");


    pub static ref WESS_READER_REQUEST_QUEUE_SIZE: IntGauge = register_int_gauge!(
        "wess_reader_queue_size",
        "Number of requests at Reader queues"
    )
    .expect("fail on registering `Number of requests at Reader queues` for prometheus");


    pub static ref WESS_RUNNER_REQUEST_QUEUE_SIZE: IntGauge = register_int_gauge!(
        "wess_runner_queue_size",
        "Number of requests at Runner queues"
    )
    .expect("fail on registering `Number of requests at Runner queues` for prometheus");


    pub static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "wess_http_requests_total",
        "Total number of HTTP requests"
    ).expect("fail on registering `Total number of HTTP requests` for prometheus");


    pub static ref HTTP_REQUEST_LATENCY: Histogram = register_histogram!(
        "wess_http_request_latency_seconds",
        "HTTP request latency in seconds"
    ).expect("fail on registering `HTTP request latency in seconds` for prometheus");


    pub static ref VIRTUAL_MEMORY_USAGE: IntGauge = register_int_gauge!(
        "wess_virtual_memory_usage",
        "Virtual memory usage in bytes"
    ).expect("fail on registering `Virtual memory usage in bytes` for prometheus");


    pub static ref DATABASE_SIZE: IntGauge = register_int_gauge!(
        "wess_database_size_bytes",
        "Size of the database in bytes"
    ).expect("fail on registering `Size of the database in bytes` for prometheus");


    pub static ref DATABASE_OPERATIONS_TOTAL: IntCounter = register_int_counter!(
        "wess_database_operations_total",
        "Total number of database operations"
    ).expect("fail on registering `Total number of database operations` for prometheus");


    pub static ref DATABASE_OPERATION_DURATION: Histogram = register_histogram!(
        "wess_database_operation_duration_seconds",
        "Duration of database operations in seconds"
    ).expect("fail on registering `Duration of database operations in seconds` for prometheus");

}
