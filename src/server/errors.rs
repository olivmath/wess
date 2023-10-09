use std::fmt;

use crate::database::models::TypeArg;

/// # Represents an error that can occur when parsing a request.
#[derive(Debug)]
pub enum RequestError {
    /// Indicates that the JSON in the request is invalid.
    ChannelError(String),
    InvalidType(TypeArg),
    InvalidJson(String),
    InvalidId(String),
    LengthArgsError {
        expect: usize,
        found: usize,
    },
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::InvalidType(t) => write!(f, "Invalid Type: cannot convert to {:?}", t),
            RequestError::ChannelError(msg) => write!(f, "Job not send: {msg}"),
            RequestError::InvalidJson(msg) => write!(f, "Invalid Json: {msg}"),
            RequestError::InvalidId(msg) => write!(f, "Invalid Id: {msg}"),
            RequestError::LengthArgsError { expect, found } => write!(
                f,
                "Wrong length of args, expect {}, found {}",
                expect, found
            ),
        }
    }
}
