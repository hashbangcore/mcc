//! Core types and services used by the CLI.
mod config;
/// CLI argument definitions.
pub mod interface;
mod router;
pub mod theme;
/// Trace server for raw LLM traffic.
pub mod trace;

pub use config::Config;
pub use interface::{Cli, Commands};
pub use router::Service;
