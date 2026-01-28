//! Hexyg Core Library
//!
//! Core functionality for bidirectional conversion between binary data and hex text format.

pub mod config;
pub mod convert;
pub mod error;

pub use config::Config;
pub use error::{Error, Result};
