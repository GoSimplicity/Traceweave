//! Shared core for Traceweave's CLI, MCP, and desktop adapters.

pub mod adapters;
pub mod application;

mod catalog;
mod chain;
mod credentials;
mod executor;
mod exploration;
mod policy;
mod postgres;
mod store;
mod workspace;
