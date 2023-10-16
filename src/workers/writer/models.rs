//! # This `models` module provides types for `writer` module
//!
//! This module contains the following types:
//!
//! - [`WriteJob`]: A struct representing a write job, containing a [`WasmModule`], a write operation type [`WriteOps`], and an ID.
//! - [`WriteOps`]: An enum representing a write operation type. It can be either create, update or delete.
//!
//! The `models` module depends on the following modules:
//!
//! - [`WasmModule`]: Represents a write request type.

use crate::database::models::WasmModule;
use core::fmt;

/// # Write Job Type
pub struct WriteJob {
    pub write_req: Option<WasmModule>,
    pub write_type: WriteOps,
    pub id: String,
}

impl WriteJob {
    /// # Creates a new [`WriteJob`] instance with the given parameters.
    ///
    /// ## Arguments
    ///
    /// * `write_req` - The [`WasmModule`] instance containing the data to be written.
    /// * `write_type` - The operation type to be executed. Can be [`WriteOps::Create`], [`WriteOps::Update`] or [`WriteOps::Delete`].
    /// * `id` - The ID of the record to be written to the database.
    ///
    /// ## Returns
    ///
    /// A new [`WriteJob`] instance containing the parameters passed as arguments.
    pub fn new(write_req: Option<WasmModule>, write_type: WriteOps, id: String) -> Self {
        Self {
            write_req,
            write_type,
            id,
        }
    }
}

/// # Write Operation Type
pub enum WriteOps {
    Create,
    Update,
    Delete,
}

pub enum WriterError {
    Create { id: String, err: String },
    Update { id: String, err: String },
    Delete { id: String, err: String },
}

impl fmt::Display for WriterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriterError::Create { id, err } => write!(f, "OP: Create, ID: {id}, Error: {err}"),
            WriterError::Update { id, err } => write!(f, "OP: Update, ID: {id}, Error: {err}"),
            WriterError::Delete { id, err } => write!(f, "OP: Delete, ID: {id}, Error: {err}"),
        }
    }
}
