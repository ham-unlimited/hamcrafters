#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Crate for handling communications with a Minecraft client.

/// Handler for client communications.
pub mod client_handler;

/// Errors that can occurr when communicating with a client.
pub mod client_error;

/// Information about the state of this client set up during config and gameplay.
pub mod client_data;
