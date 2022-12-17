#![feature(async_fn_in_trait)]

/// The API Implementation (Endpoints)
mod api;
/// Client type
pub mod client;
/// Possible error types
mod error;
/// Tests for types and client including a mock
#[cfg(test)]
mod tests;
/// Types for the `cryptowatch` API
pub mod types;
pub use error::Error;

pub mod prelude {
	pub use super::error::*;
	use super::*;
	pub use api::*;
	pub use client::*;
	pub use types::*;
}
