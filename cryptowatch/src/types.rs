use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod assets;
mod exchanges;
mod markets;
mod pairs;

pub use assets::*;
pub use exchanges::*;
pub use markets::*;
pub use pairs::*;

/// The general REST API "Response" type.
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
	result: T,
	#[serde(default)]
	cursor: Cursor,
	allowance: ApiAllowance<'static>,
}

impl<T> Response<T> {
	/// unpack the `[Response]` yeilding its associated body, `T`.
	pub fn unpack(self) -> T {
		self.result
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cursor {
	last: Option<String>,
	#[serde(rename = "hasMore")]
	has_more: bool,
}

impl Default for Cursor {
	fn default() -> Self {
		Cursor { last: None, has_more: false }
	}
}

/// Information about the amount of credits left before a user reaches their rate-limit.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiAllowance<'a> {
	cost: f64,
	remaining: f64,
	upgrade: Cow<'a, str>,
}

impl<'a> std::fmt::Display for ApiAllowance<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"This api call cost {} and you have {} credits remaining. {}",
			self.cost, self.remaining, self.upgrade
		)
	}
}
