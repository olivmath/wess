use crate::metrics::constants::{HTTP_REQUESTS_TOTAL, HTTP_REQUEST_LATENCY};
use tide::{Middleware, Next, Request, Result};

pub struct RequestMetricsMiddleware;

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for RequestMetricsMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        HTTP_REQUESTS_TOTAL.inc();

        let r = HTTP_REQUEST_LATENCY
            .observe_closure_duration(|| async { next.run(req).await })
            .await;

        Ok(r)
    }
}