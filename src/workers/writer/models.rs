//! # This `models` module provides types for `writer` module
//!
//! This module contains the following types:
//!
//! - [`WJob`]: A struct representing a write job, containing a [`WRequest`], a write operation type [`WOps`], and an ID.
//! - [`WOps`]: An enum representing a write operation type. It can be either create, update or delete.
//!
//! The `models` module depends on the following modules:
//!
//! - [`WRequest`]: Represents a write request type.

use crate::server::models::WRequest;
use core::fmt;

/// # Write Job Type
pub struct WJob {
    pub wreq: WRequest,
    pub wtype: WOps,
    pub id: String,
}

impl WJob {
    /// # Creates a new [`WJob`] instance with the given parameters.
    ///
    /// ## Arguments
    ///
    /// * `wreq` - The [`WRequest`] instance containing the data to be written.
    /// * `wtype` - The operation type to be executed. Can be [`WOps::Create`], [`WOps::Update`] or [`WOps::Delete`].
    /// * `id` - The ID of the record to be written to the database.
    ///
    /// ## Returns
    ///
    /// A new [`WJob`] instance containing the parameters passed as arguments.
    pub fn new(wreq: WRequest, wtype: WOps, id: String) -> Self {
        Self { wreq, wtype, id }
    }
}

/// # Write Operation Type
pub enum WOps {
    Create,
    Update,
    Delete,
}

pub enum WriterError {
    Create(String, String),
    Update(String, String),
    Delete(String, String),
}

impl fmt::Display for WriterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriterError::Create(id, err) => write!(f, "Create: {id} Error: {err}"),
            WriterError::Update(id, err) => write!(f, "Update: {id} Error: {err}"),
            WriterError::Delete(id, err) => write!(f, "Delete: {id} Error: {err}"),
        }
    }
}
