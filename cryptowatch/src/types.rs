use hyper::Uri;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

mod assets;
mod exchanges;
mod impls;
mod markets;
mod pairs;

pub use assets::*;
pub use exchanges::*;
pub use impls::*;
pub use markets::*;
pub use pairs::*;

/// The general REST API "Response" type.
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
	pub(crate) result: T,
	#[serde(default)]
	pub(crate) cursor: Cursor,
	allowance: ApiAllowance<'static>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cursor {
	pub(crate) last: Option<String>,
	#[serde(rename = "hasMore")]
	pub(crate) has_more: bool,
}

/// Information about the amount of credits left before a user reaches their rate-limit.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiAllowance<'a> {
	pub(crate) cost: f64,
	pub(crate) remaining: f64,
	pub(crate) upgrade: Cow<'a, str>,
}

/// A route to other APIs associated with this call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route(pub Uri);

/// Wrapper around a f64 to represent a price.
#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
	price: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum RouteType {
	#[serde(rename = "price")]
	Price,
	#[serde(rename = "summary")]
	Summary,
	#[serde(rename = "orderbook")]
	Orderbook,
	#[serde(rename = "trades")]
	Trades,
	#[serde(rename = "ohlc")]
	OHLC,
	#[serde(rename = "markets")]
	Markets,
}

//TODO: Can deserialize this into Vec<Endpoint> instead of this enum. It would make
// these routes much more useful, since once could actually query them immediately.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SingleOrMultipleRoutes {
	#[serde(rename = "route")]
	Single(Route),
	#[serde(rename = "routes")]
	Multiple(HashMap<String, Route>),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_deserialize_str_as_uri() {
		#[derive(Deserialize)]
		struct TestStruct {
			route: Route,
		}

		let test_uri = "https://api.cryptowat.ch/markets/kraken/btccad".parse::<hyper::Uri>().unwrap();
		let uri = serde_json::json!({ "route": "https://api.cryptowat.ch/markets/kraken/btccad" });
		let test: TestStruct = serde_json::from_value(uri).unwrap();

		assert_eq!(*test.route, test_uri);
	}

	#[test]
	fn test_serialize_uri_as_str() {
		#[derive(Serialize)]
		struct TestStruct {
			route: Route,
		}

		let test_uri = "https://api.cryptowat.ch/markets/kraken/btccad".parse::<hyper::Uri>().unwrap();
		let test = TestStruct { route: Route(test_uri) };
		let uri = serde_json::json!({ "route": "https://api.cryptowat.ch/markets/kraken/btccad" });

		assert_eq!(serde_json::to_value(test).unwrap(), uri);
	}
}
