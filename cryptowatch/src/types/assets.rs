use serde::{Deserialize, Serialize};

use super::*;

/// An single asset
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Asset {
	pub id: u64,
	pub sid: String,
	pub symbol: String,
	pub name: String,
	pub fiat: bool,
	pub route: Route,
}

/// Details about an asset.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AssetDetails {
	pub id: u64,
	pub sid: String,
	pub symbol: String,
	pub name: String,
	pub fiat: bool,
	pub markets: CurrencyPair,
}

///  Base/Quote for an Asset
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CurrencyPair {
	pub base: Vec<Market>,
	pub quote: Vec<Market>,
}

#[cfg(test)]
mod tests {
	use crate::{
		tests::{data_prelude::*, prelude::*},
		types::*,
	};

	// assets -> List
	#[tokio::test]
	async fn test_real_data_asset_deserialization() {
		let assets = test_data(Call::Assets(Assets::List)).await;
		let _: Response<Vec<Asset>> = assert_ok!(serde_json::from_slice(assets.as_slice()));
	}

	#[tokio::test]
	async fn test_real_data_asset_details_deserialization() {
		let assets = test_data(Call::Assets(Assets::Details)).await;
		let _: Response<AssetDetails> = assert_ok!(serde_json::from_slice(assets.as_slice()));
	}
}
