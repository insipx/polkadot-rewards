//! Types related to market pairs.
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
/// A single Pair between two assets. EX: "btceur"
#[serde(transparent)]
pub struct Pair {
	pub pair: String,
}

/// An asset/asset pairing.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PairInfo {
	id: u64,
	symbol: String,
	base: Asset,
	quote: Asset,
	route: Route,
}

/// General details about a pair.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PairDetails {
	id: u64,
	symbol: String,
	base: Asset,
	quote: Asset,
	route: Route,
	markets: Vec<Market>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{data_prelude::*, prelude::*};

	#[tokio::test]
	async fn test_real_data_pair_deserialization() {
		let pairs = test_data(Call::Pairs(Pairs::List)).await;
		let _: Response<Vec<PairInfo>> = assert_ok!(serde_json::from_slice(pairs.as_slice()));
	}

	#[tokio::test]
	async fn test_real_data_pair_details_deserialization() {
		let pairs = test_data(Call::Pairs(Pairs::Details)).await;
		let _: Response<PairDetails> = assert_ok!(serde_json::from_slice(pairs.as_slice()));
	}
}
