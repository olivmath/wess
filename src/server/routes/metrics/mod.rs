use prometheus::{Encoder, TextEncoder};
use tide::{http::Mime, Error, Response, StatusCode};

pub async fn prometheus_metrics() -> Result<Response, Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    let text_plain_utf_8 = "text/plain; version=0.0.4; charset=utf-8"
        .parse::<Mime>()
        .unwrap();

    encoder.encode(&metric_families, &mut buffer)?;

    let response = tide::Response::builder(StatusCode::Ok)
        .body(tide::Body::from(buffer))
        .content_type(text_plain_utf_8)
        .build();

    Ok(response)
}
