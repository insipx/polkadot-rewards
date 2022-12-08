use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

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
	allowance: ApiAllowance<'static>,
}

impl<T> Response<T> {
	/// unpack the `[Response]` yeilding its associated body, `T`.
	pub fn unpack(self) -> T {
		self.result
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
