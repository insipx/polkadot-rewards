use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use super::*;

/// An single asset
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset<'a> {
	pub id: u64,
	#[serde(borrow)]
	pub sid: Cow<'a, str>,
	#[serde(borrow)]
	pub symbol: Cow<'a, str>,
	#[serde(borrow)]
	pub name: Cow<'a, str>,
	pub fiat: bool,
	pub route: Route,
}

/// Details about an asset.
#[derive(Serialize, Deserialize, Debug)]
pub struct AssetDetails<'a> {
	pub id: u64,
	#[serde(borrow)]
	pub sid: Cow<'a, str>,
	#[serde(borrow)]
	pub symbol: Cow<'a, str>,
	#[serde(borrow)]
	pub name: Cow<'a, str>,
	pub fiat: bool,
	#[serde(borrow)]
	pub markets: CurrencyPair<'a>,
}

///  Base/Quote for an Asset
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPair<'a> {
	#[serde(borrow)]
	pub base: Vec<MarketAsset<'a>>,
	#[serde(borrow)]
	pub quote: Vec<MarketAsset<'a>>,
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
