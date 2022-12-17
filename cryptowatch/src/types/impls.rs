//! Implementations for general types.

use crate::{error::Error, types::*};
use hyper::Uri;
use serde::{de, ser};
use std::str::FromStr;

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

	/// Get the allowance left using this API key or the public allowance.
	pub fn allowance(self) -> ApiAllowance<'static> {
		self.allowance
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

impl FromStr for RouteType {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"price" => Ok(RouteType::Price),
			"summary" => Ok(RouteType::Summary),
			"orderbook" => Ok(RouteType::Orderbook),
			"trades" => Ok(RouteType::Trades),
			"ohlc" => Ok(RouteType::OHLC),
			"markets" => Ok(RouteType::Markets),
			_ => Err(Error::InvalidString(s.to_string())),
		}
	}
}

impl ToString for RouteType {
	fn to_string(&self) -> String {
		match self {
			RouteType::Price => "price".to_string(),
			RouteType::Summary => "summary".to_string(),
			RouteType::Orderbook => "orderbook".to_string(),
			RouteType::Trades => "trades".to_string(),
			RouteType::OHLC => "ohlc".to_string(),
			RouteType::Markets => "markets".to_string(),
		}
	}
}
