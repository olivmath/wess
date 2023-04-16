//! # Contains the implementation of the server routes used to handle incoming requests.
//!
//! The routes are implemented as asynchronous functions that take a [`Request`]
//! object and return a [`Result`] containing the HTTP response.
//!
//!
//! The server routes are implemented as follows:
//!
//! * [`write_op`] - handles write operations (create, update, delete) on a WebAssembly function.
//! * [`read_op`] - handles read operations (get) on a WebAssembly function.
//! * [`run_op`] - handles requests to run a WebAssembly function.
//!
//! All routes take a [`Request`] object that provides access to the HTTP request data and a
//! [`AppState`] object that contains the application state (i.e., the channels used to communicate
//! with the worker threads).
//!
//! The [`write_op`] function is used to create, update or delete a WebAssembly function.
//! It extracts the request data and passes it to the [`send_to_writer`] function to be
//! sent to the writer thread.
//!
//! The [`read_op`] function is used to get the WebAssembly bytecode for a given function ID.
//! It passes the ID to the [`send_to_reader`] function to be sent to the reader thread.
//!
//! The [`run_op`] function is used to run a WebAssembly function. It extracts the request data and
//! passes it to the [`send_to_runner`] function to be sent to the runner thread.

pub mod metrics;
pub mod read_ops;
pub mod run_ops;
pub mod write_ops;
