use hyper::Uri;
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
	pub(crate) result: T,
	#[serde(default)]
	cursor: Cursor,
	allowance: ApiAllowance<'static>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route(pub Uri);

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
