use std::time::Instant;

use crate::metrics::constants::{HTTP_REQUESTS_TOTAL, HTTP_REQUEST_LATENCY};
use tide::{Middleware, Next, Request, Result};

pub struct RequestMetricsMiddleware;

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequestMetricsMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        HTTP_REQUESTS_TOTAL.inc();

        let start_time = Instant::now();
        let response = next.run(req).await;
        let elapsed_time = start_time.elapsed().as_secs_f64();

        HTTP_REQUEST_LATENCY.observe(elapsed_time);

        Ok(response)
    }
}
