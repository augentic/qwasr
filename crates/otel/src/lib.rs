#![cfg(not(target_arch = "wasm32"))]

//! # Telemtry
//!
//! Telemetry is a module that provides functionality for collecting and
//! reporting OpenTelemetry-based metrics.

pub mod init;
pub mod tracing;

pub use init::Telemetry;
pub use tracing::*;
