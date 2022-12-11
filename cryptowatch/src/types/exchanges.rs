use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeDetails<'a> {
	id: u64,
	#[serde(borrow)]
	symbol: Cow<'a, str>,
	#[serde(borrow)]
	name: Cow<'a, str>,
	active: bool,
	#[serde(borrow)]
	#[serde(flatten)]
	routes: SingleOrMultipleRoutes<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SingleOrMultipleRoutes<'a> {
	#[serde(borrow)]
	#[serde(rename = "route")]
	Single(Cow<'a, str>),
	#[serde(borrow)]
	#[serde(rename = "routes")]
	Multiple(HashMap<Cow<'a, str>, Cow<'a, str>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{tests::test_prelude::*, types::*};

	#[test]
	fn test_exchange_list_deserialization() {
		let list = load_test_data(Call::Exchanges(Exchanges::List));
		let _: Response<Vec<ExchangeDetails>> = assert_ok!(serde_json::from_slice(list.as_slice()));
	}

	#[test]
	fn test_exchange_details_deserialization() {
		let details = load_test_data(Call::Exchanges(Exchanges::Details));
		let _: Response<ExchangeDetails> = assert_ok!(serde_json::from_slice(details.as_slice()));
	}

	#[test]
	fn test_exchange_market_deserialization() {
		let market = load_test_data(Call::Exchanges(Exchanges::Markets));
		let _: Response<Vec<MarketAsset>> = assert_ok!(serde_json::from_slice(market.as_slice()));
	}
}
