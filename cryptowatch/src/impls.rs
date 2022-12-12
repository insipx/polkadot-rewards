//! Implementations for general types.

use crate::types::*;
use hyper::Uri;
use serde::{de, ser};

mod exchanges;
mod markets;
mod pairs;

pub use exchanges::*;
pub use markets::*;
pub use pairs::*;

impl<T> Response<T> {
	/// unpack the `[Response]` yeilding its associated body, `T`.
	pub fn unpack(self) -> T {
		self.result
	}
}

impl Default for Cursor {
	fn default() -> Self {
		Cursor { last: None, has_more: false }
	}
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

impl std::ops::Deref for Route {
	type Target = Uri;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'de> de::Deserialize<'de> for Route {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		let uri = String::deserialize(deserializer)?;
		uri.parse::<Uri>().map(Route).map_err(de::Error::custom)
	}
}

impl ser::Serialize for Route {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: ser::Serializer,
	{
		serializer.serialize_str(&self.0.to_string())
	}
}
