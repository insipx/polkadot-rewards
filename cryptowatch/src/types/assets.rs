use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
	#[serde(borrow)]
	route: Cow<'a, str>,
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
}
