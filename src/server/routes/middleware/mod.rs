use std::time::Instant;

use crate::metrics::constants::{
    CONCURRENT_CONNECTIONS, HTTP_REQUESTS_TOTAL, HTTP_REQUEST_LATENCY,
};
use tide::{Middleware, Next, Request, Result};

pub struct RequestMetricsMiddleware;

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequestMetricsMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        HTTP_REQUESTS_TOTAL.inc();

        let start = Instant::now();
        CONCURRENT_CONNECTIONS.inc();

        let r = next.run(req).await;

        CONCURRENT_CONNECTIONS.dec();
        let duration = start.elapsed();

        HTTP_REQUEST_LATENCY.observe(duration.as_secs_f64());

        Ok(r)
    }
}
