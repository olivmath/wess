use serde::Serialize;
use std::{convert::TryFrom, fmt::Display};
use tide::StatusCode;

#[derive(Serialize, Debug)]
pub struct WessError {
    pub msg: String,
    pub status: StatusCode,
}

impl WessError {
    pub fn new(msg: String, status: u16) -> WessError {
        match StatusCode::try_from(status) {
            Ok(status) => WessError { msg, status },
            Err(_) => WessError::new(msg, 500),
        }
    }
}

impl Display for WessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}
