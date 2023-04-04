use std::fmt;

/// # Represents an error that can occur when parsing a request.
#[derive(Debug)]
pub enum RequestError {
    /// Indicates that the JSON in the request is invalid.
    ChannelError(String),
    InvalidJson(String),
    InvalidId(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::ChannelError(msg) => write!(f, "Job not send: {msg}"),
            RequestError::InvalidJson(msg) => write!(f, "Invalid Json: {msg}"),
            RequestError::InvalidId(msg) => write!(f, "Invalid Id: {msg}"),
        }
    }
}
