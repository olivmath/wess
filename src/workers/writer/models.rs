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

/// # Write Job Type
pub struct WriteJob {
    pub write_op: WriteOps,
    pub write_module: Option<WasmModule>,
    pub id: String,
}

impl WriteJob {
    /// # Creates a new [`WriteJob`] instance with the given parameters.
    ///
    /// ## Arguments
    ///
    /// * `write_module` - The [`WasmModule`] instance containing the data to be written.
    /// * `write_type` - The operation type to be executed. Can be [`WriteOps::Create`], [`WriteOps::Update`] or [`WriteOps::Delete`].
    /// * `id` - The ID of the record to be written to the database.
    ///
    /// ## Returns
    ///
    /// A new [`WriteJob`] instance containing the parameters passed as arguments.
    pub fn new(write_module: Option<WasmModule>, id: String, write_op: WriteOps) -> Self {
        Self {
            write_module,
            write_op,
            id,
        }
    }
}

/// # Write Operation Type
#[derive(Clone)]
pub enum WriteOps {
    Create,
    Update,
    Delete,
}
