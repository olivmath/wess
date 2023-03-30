use serde::Serialize;
use serde_json::json;
use tide::{Error, Response, StatusCode};

/// # Returns an error response with the provided message.
///
/// ## Arguments
///
/// * `message` - The error message to include in the response body.
///
/// ## Returns
///
/// * A [`Result`] containing the error response.
pub async fn respond_with_error(message: String) -> Result<Response, Error> {
    Ok(Response::builder(StatusCode::InternalServerError)
        .body(json!({ "message": message }))
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
pub async fn respond<T>(message: T) -> Result<Response, Error>
where
    T: Serialize,
{
    Ok(Response::builder(StatusCode::Ok)
        .body(json!({ "message": message }))
        .build())
}
