#![feature(async_fn_in_trait)]

/// Client type
pub mod client;
/// Possible error types
mod error;
/// Implementors for types and Client
mod impls;
/// Tests for types and client including a mock
#[cfg(test)]
mod tests;
/// Types for the `cryptowatch` API
pub mod types;

pub mod prelude {
	use super::*;
	pub use client::*;
	pub use error::*;
	pub use impls::*;
	pub use types::*;
}
