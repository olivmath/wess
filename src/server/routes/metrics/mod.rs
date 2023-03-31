use prometheus::{Encoder, TextEncoder, TEXT_FORMAT};
use tide::{Error, Response, StatusCode};

pub async fn prometheus_metrics() -> Result<Response, Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer)?;
    let metrics = String::from_utf8(buffer).unwrap();

    let response = tide::Response::builder(StatusCode::Ok)
        .body(metrics)
        .content_type(TEXT_FORMAT)
        .build();

    Ok(response)
}
