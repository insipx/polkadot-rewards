use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::*;

/// An single asset
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset<'a> {
	id: u64,
	#[serde(borrow)]
	sid: Cow<'a, str>,
	#[serde(borrow)]
	symbol: Cow<'a, str>,
	#[serde(borrow)]
	name: Cow<'a, str>,
	fiat: bool,
	route: Route,
}

/// Details about an asset.
#[derive(Serialize, Deserialize, Debug)]
pub struct AssetDetails<'a> {
	id: u64,
	#[serde(borrow)]
	sid: Cow<'a, str>,
	#[serde(borrow)]
	symbol: Cow<'a, str>,
	#[serde(borrow)]
	name: Cow<'a, str>,
	fiat: bool,
	#[serde(borrow)]
	markets: CurrencyPair<'a>,
}

///  Base/Quote for an Asset
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPair<'a> {
	#[serde(borrow)]
	base: Vec<MarketAsset<'a>>,
	#[serde(borrow)]
	quote: Vec<MarketAsset<'a>>,
}

#[cfg(test)]
mod tests {
	use crate::{tests::test_prelude::*, types::*};

	// assets -> List
	#[test]
	fn test_real_data_asset_deserialization() {
		let assets = load_test_data(Call::Assets(Assets::List));
		let _: Response<Vec<Asset>> = assert_ok!(serde_json::from_slice(assets.as_slice()));
	}

	#[test]
	fn test_real_data_asset_details_deserialization() {
		let assets = load_test_data(Call::Assets(Assets::Details));
		let _: Response<AssetDetails> = assert_ok!(serde_json::from_slice(assets.as_slice()));
	}
}
