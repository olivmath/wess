//! # This module contains the worker components that are responsible for reading, running, and writing WebAssembly functions in the database.
//!
//! There are three sub-modules:
//!
//! - [`reader`]: Reads a WebAssembly function from the database.
//! - [`runner`]: Runs a WebAssembly function.
//! - [`writer`]: Writes a WebAssembly function to the database.
//!
//! Each worker runs on its own asynchronous task.
//!
//! The workers depend on the following modules:
//!
//! - [`crate::server::request`]: Defines the types of incoming requests to the server.
//! - [`crate::database`]: Provides the database layer where the WebAssembly functions are stored.
//! - [`tokio::sync`]: Provides the concurrency primitives used to communicate between the workers.
//!
//! [`reader`]: reader/mod.rs
//! [`runner`]: runner/mod.rs
//! [`writer`]: writer/mod.rs


pub mod reader;
pub mod runner;
pub mod writer;
