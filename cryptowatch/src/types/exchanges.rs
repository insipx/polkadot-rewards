use super::*;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Exchange {
	#[serde(rename = "bitflyer")]
	BitFlyer,
	#[serde(rename = "bittrex")]
	Bittrex,
	#[serde(rename = "gemini")]
	Gemini,
	#[serde(rename = "luno")]
	Luno,
	#[serde(rename = "gateio")]
	Gateio,
	#[serde(rename = "bitfinex")]
	Bitfinex,
	#[serde(rename = "kraken")]
	Kraken,
	#[serde(rename = "cexio")]
	Cexio,
	#[serde(rename = "bisq")]
	Bisq,
	#[serde(rename = "bitmex")]
	BitMEX,
	#[serde(rename = "okex")]
	Okex,
	#[serde(rename = "kraken-futures")]
	KrakenFutures,
	#[serde(rename = "liquid")]
	Liquid,
	#[serde(rename = "quoine")]
	Quoine,
	#[serde(rename = "bitbay")]
	BitBay,
	#[serde(rename = "hitbtc")]
	HitBTC,
	#[serde(rename = "binance")]
	Binance,
	#[serde(rename = "binance-us")]
	BinanceUS,
	#[serde(rename = "huobi")]
	Huobi,
	#[serde(rename = "poloniex")]
	Poloniex,
	#[serde(rename = "coinbase-pro")]
	CoinbasePro,
	#[serde(rename = "bitstamp")]
	Bitstamp,
	#[serde(rename = "bitz")]
	BitZ,
	#[serde(rename = "bithumb")]
	Bithumb,
	#[serde(rename = "coinone")]
	Coinone,
	#[serde(rename = "dex-aggregated")]
	DexAggregated,
	#[serde(rename = "okcoin")]
	OkCoin,
	#[serde(rename = "ftx")]
	Ftx,
	#[serde(rename = "uniswap-v2")]
	UniswapV2,
	Unknown(String),
}

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
	#[serde(rename = "route")]
	Single(Cow<'a, Route>),
	#[serde(borrow)]
	#[serde(rename = "routes")]
	Multiple(HashMap<Cow<'a, str>, Cow<'a, Route>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::test_prelude::*;

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
