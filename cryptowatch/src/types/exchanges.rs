use super::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(from = "String")]
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
	#[serde(alias = "Bitfinex")]
	#[serde(rename = "bitfinex")]
	Bitfinex,
	#[serde(alias = "Kraken")]
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
	#[serde(rename = "bybit")]
	Bybit,
	#[serde(rename = "crypto-com")]
	CryptoCom,
	#[serde(rename = "deribit")]
	Deribit,
	#[serde(rename = "kucoin")]
	KuCoin,
	#[serde(rename = "okx")]
	Okx,
	#[serde(rename = "zonda")]
	Zonda,
	#[serde(rename = "wex")]
	Wex,
	#[serde(rename = "cryptsy")]
	Cryptsy,
	Other(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeDetails {
	id: u64,
	symbol: Exchange,
	/// Common name for this exchange. Formatting differs.
	name: String,
	active: bool,
	#[serde(flatten)]
	routes: SingleOrMultipleRoutes,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{data_prelude::*, prelude::*};

	#[tokio::test]
	async fn test_exchange_list_deserialization() {
		let list = test_data(Call::Exchanges(Exchanges::List)).await;
		let _: Response<Vec<ExchangeDetails>> = assert_ok!(serde_json::from_slice(list.as_slice()));
	}

	#[tokio::test]
	async fn test_exchange_details_deserialization() {
		let details = test_data(Call::Exchanges(Exchanges::Details)).await;
		let _: Response<ExchangeDetails> = assert_ok!(serde_json::from_slice(details.as_slice()));
	}

	#[tokio::test]
	async fn test_exchange_market_deserialization() {
		let market = test_data(Call::Exchanges(Exchanges::Markets)).await;
		let _: Response<Vec<Market>> = assert_ok!(serde_json::from_slice(market.as_slice()));
	}
}
