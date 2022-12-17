//! Types related to market pairs.
use super::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
/// A single Pair between two assets. EX: "btceur"
#[serde(transparent)]
pub struct Pair<'a> {
	pub(crate) pair: Cow<'a, str>,
}

/// An asset/asset pairing.
#[derive(Serialize, Deserialize, Debug)]
pub struct PairInfo {
	id: u64,
	symbol: String,
	base: Asset,
	quote: Asset,
	route: Route,
}

/// General details about a pair.
#[derive(Serialize, Deserialize, Debug)]
pub struct PairDetails {
	id: u64,
	symbol: String,
	base: Asset,
	quote: Asset,
	route: Route,
	markets: Vec<MarketAsset>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::test_prelude::*;

	#[test]
	fn test_real_data_pair_deserialization() {
		let pairs = load_test_data(Call::Pairs(Pairs::List));
		let _: Response<Vec<PairInfo>> = assert_ok!(serde_json::from_slice(pairs.as_slice()));
	}

	#[test]
	fn test_real_data_pair_details_deserialization() {
		let pairs = load_test_data(Call::Pairs(Pairs::Details));
		let _: Response<PairDetails> = assert_ok!(serde_json::from_slice(pairs.as_slice()));
	}
}