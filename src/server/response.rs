use crate::errors::WessError;
use serde::Serialize;
use serde_json::json;
use tide::{Error, Response, StatusCode};

/// TODO
/// join this functions into a module called Responder
/// put this new module such as a deps to pass into `performers_ops`
///
/// # example:
/// ```rust
/// pub async fn make_run_op(mut req: Request<AppState>, response: Responder) -> Result<Response, Error> {
///     match result {
///         Ok(data) => response.success(data).await
///         Err(err) => response.fail(err).await
///     }
/// }
/// ```

/// # Returns an error response with the provided message.
///
/// ## Arguments
///
/// * `message` - The error message to include in the response body.
///
/// ## Returns
///
/// * A [`Result`] containing the error response.
pub async fn respond_with_error(error: WessError) -> Result<Response, Error> {
    Ok(Response::builder(error.status)
        .body(json!({ "message": error.msg }))
        .build())
}

/// # Returns a successful response with the provided message.
///
/// ## Arguments
///
/// * `message` - The message to include in the response body.
///
/// ## Returns
///
/// * A [`Result`] containing the successful response.
pub async fn respond<T>(message: T, status: StatusCode) -> Result<Response, Error>
where
    T: Serialize,
{
    Ok(Response::builder(status)
        .body(json!({ "message": message }))
        .build())
}
