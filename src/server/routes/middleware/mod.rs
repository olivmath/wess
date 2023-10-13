use std::time::Instant;

use crate::metrics::constants::{
    CONCURRENT_CONNECTIONS, HTTP_REQUESTS_TOTAL, HTTP_REQUEST_LATENCY,
};
use tide::{Middleware, Next, Request, Result};

pub struct RequestMetricsMiddleware;

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequestMetricsMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        HTTP_REQUESTS_TOTAL
            .with_label_values(&[req.url().path()])
            .inc();
        CONCURRENT_CONNECTIONS.inc();

        let start = Instant::now();
        let response = next.run(req).await;
        let duration = start.elapsed();

        CONCURRENT_CONNECTIONS.dec();
        HTTP_REQUEST_LATENCY.observe(duration.as_secs_f64());

        Ok(response)
    }
}
