use std::fmt;

/// # Represents an error that can occur when parsing a request.
#[derive(Debug)]
pub enum RequestError {
    /// Indicates that the JSON in the request is invalid.
    InvalidJson(String),
    ChannelError(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::InvalidJson(message) => write!(f, "Invalid Json: {message}"),
            RequestError::ChannelError(message) => {
                write!(f, "Could not send the job through the channel: {message}")
            }
        }
    }
}
