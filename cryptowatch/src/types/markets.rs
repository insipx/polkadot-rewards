//! Types representing Markets.
use super::*;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::{borrow::Cow, collections::HashMap};

/// An asset describing a market
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketAsset<'a> {
	id: u64,
	exchange: Cow<'a, str>,
	pair: Cow<'a, str>,
	active: bool,
	// TODO: Maybe accept a URL type for Route
	route: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketAssetDetails<'a> {
	id: u64,
	exchange: Exchange,
	#[serde(borrow)]
	pair: Pair<'a>,
	active: bool,
	#[serde(borrow)]
	routes: MarketRoutes<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketRoutes<'a> {
	#[serde(borrow)]
	price: Cow<'a, str>,
	#[serde(borrow)]
	summary: Cow<'a, str>,
	#[serde(borrow)]
	orderbook: Cow<'a, str>,
	#[serde(borrow)]
	trades: Cow<'a, str>,
	#[serde(borrow)]
	ohlc: Cow<'a, str>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InvestmentVehicle {
	Market,
	Index,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Exchange {
	#[serde(rename = "kraken")]
	Kraken,
	#[serde(rename = "kraken-futures")]
	KrakenFutures,
	#[serde(rename = "binance-us")]
	BinanceUS,
	#[serde(rename = "binance")]
	BinanceIntl,
	#[serde(rename = "coinbase-pro")]
	Coinbase,
	Other(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnitPrice {
	price: f64,
}

#[derive(Debug)]
pub struct PriceAndInfoInner {
	pub investment_vehicle: InvestmentVehicle,
	pub exchange: Exchange,
	pub pair: String,
	pub price: f64,
}

pub struct PriceAndInfo {
	pub inner: Vec<PriceAndInfoInner>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
	id: u64,
	#[serde(with = "ts_seconds")]
	timestamp: DateTime<Utc>,
	price: f64,
	amount: f64,
}

/// Summary of the market for one asset in a 24-hour time period
#[derive(Serialize, Deserialize, Debug)]
pub struct MarketSummary {
	/// Price of the asset
	price: PriceSummary,
	/// Volume of the base asset
	volume: f64,
	/// Volume of the quote asset
	#[serde(rename = "volumeQuote")]
	volume_quote: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceSummary {
	last: f64,
	high: f64,
	low: f64,
	change: PriceChange,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceChange {
	percentage: f64,
	absolute: f64,
}

#[derive(Serialize, Debug)]
pub struct AllMarketSummaries<'a> {
	pub(crate) inner: HashMap<(Exchange, Pair<'a>), MarketSummary>,
}

/// An individual offer to either sell or buy an `amount` of an asset at a `price`.
#[derive(Debug, Serialize)]
pub struct Offer {
	pub(crate) price: f64,
	pub(crate) amount: f64,
}

/// The order book for a given market
#[derive(Serialize, Deserialize, Debug)]
pub struct Orderbook {
	/// Asks for an asset. The lowest price a seller is willing to sell the asset.
	asks: Vec<Offer>,
	/// Bids for an asset. The highest price a buyer is willing to buy the asset.
	bids: Vec<Offer>,
	/// Intended for use with the WebSockets API. Used to resynchronize the order book and replay deltas received over
	/// the live feed which have a higher [`sequence_number`].
	#[serde(rename = "seqNum")]
	sequence_number: u64,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
#[serde(transparent)]
pub(crate) struct F64(pub f64);

/// Sums of the base and quote assets at different point levels.
#[derive(Serialize, Deserialize, Debug)]
pub struct LiquiditySums {
	/// The base asset
	// TODO: What is the 'base' asset?
	base: HashMap<BasisPointLevel, F64>,
	/// quote asset
	// TODO: What is the 'quote' asset?
	quote: HashMap<BasisPointLevel, F64>,
}

// TODO: I'm not really sure what this means. fill it in.
/// The Basis Point Level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u64)]
pub enum BasisPointLevel {
	TwentyFive = 25u64,
	Fifty = 50u64,
	SeventyFive = 75u64,
	OneHundred = 100u64,
	OneHundredFifty = 150u64,
	TwoHundred = 200u64,
	TwoHundredFifty = 250u64,
	ThreeHundred = 300u64,
	FourHundred = 400u64,
	FiveHundred = 500u64,
}

/// Liquidity sums at several basis point levels in the order book.
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookLiquidity {
	/// Asks for an asset. The lowest price a seller is willing to sell the asset.
	ask: LiquiditySums,
	/// Bids for an asset. The highest price a buyer is willing to buy the asset.
	bid: LiquiditySums,
}

/// Provides a live quote from the order book for a given buy & sell amount
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookCalculator {
	buy: OrderbookQuote,
	sell: OrderbookQuote,
}

/// Average price and deltas of a sell or buy
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookQuote {
	#[serde(rename = "avgPrice")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	avg_price: f64,
	#[serde(rename = "avgDelta")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	avg_delta: f64,
	#[serde(rename = "avgDeltaBps")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	avg_delta_bps: f64,
	#[serde(rename = "reachPrice")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	reach_price: f64,
	#[serde(rename = "reachDelta")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	reach_delta: f64,
	#[serde(rename = "reachDeltaBps")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	reach_delta_bps: f64,
	/// The use of this quote, one of [`SpendOrReceive::Spend`] or [`SpendOrReceive::Receive`]
	#[serde(flatten)]
	r#use: SpendOrReceive,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SpendOrReceive {
	#[serde(rename = "spend")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	Spend(f64),
	#[serde(rename = "receive")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	Receive(f64),
}

/// Mapping of Period to the [`OHLC`] data for that length of time
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PeriodMap(HashMap<Period, Vec<OHLC>>);

/// The candle length ("Period") in seconds. Ranges from 1 minute to 1 week.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Period {
	#[serde(rename = "60")]
	P60,
	#[serde(rename = "180")]
	P180,
	#[serde(rename = "300")]
	P300,
	#[serde(rename = "900")]
	P900,
	#[serde(rename = "1800")]
	P1800,
	#[serde(rename = "3600")]
	P3600,
	#[serde(rename = "7200")]
	P7200,
	#[serde(rename = "14400")]
	P14400,
	#[serde(rename = "21600")]
	P21600,
	#[serde(rename = "43200")]
	P43200,
	#[serde(rename = "86400")]
	P86400,
	#[serde(rename = "259200")]
	P259200,
	#[serde(rename = "604800")]
	P604800,
	#[serde(rename = "604800_Monday")]
	P604800Monday,
}

/// "OHLC" data for a period of time. "OHLC" stands for "Open-High-Low-Close"
#[derive(Serialize, Deserialize, Debug)]
pub struct OHLC {
	#[serde(with = "ts_seconds")]
	close_time: DateTime<Utc>,
	open_price: f64,
	high_price: f64,
	low_price: f64,
	close_price: f64,
	volume: f64,
	quote_volume: f64,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::test_prelude::*;

	#[test]
	fn test_market_list_deserialization() {
		let list = load_test_data(Call::Markets(Market::List));
		let _: Response<Vec<MarketAsset>> = assert_ok!(serde_json::from_slice(list.as_slice()));
	}

	#[test]
	fn test_market_details_deserialization() {
		let details = load_test_data(Call::Markets(Market::Details));
		let _: Response<MarketAssetDetails> = assert_ok!(serde_json::from_slice(details.as_slice()));
	}

	#[test]
	fn test_market_price_deserialization() {
		let price = load_test_data(Call::Markets(Market::Price(OneOrAllMarkets::One)));
		let _: Response<UnitPrice> = assert_ok!(serde_json::from_slice(price.as_slice()));
	}

	#[test]
	fn test_all_market_price_deserialization() {
		let price = load_test_data(Call::Markets(Market::Price(OneOrAllMarkets::All)));
		let _: Response<PriceAndInfo> = assert_ok!(serde_json::from_slice(price.as_slice()));
	}

	#[test]
	fn test_trades_deserialization() {
		let trades = load_test_data(Call::Markets(Market::Trades));
		let _: Response<Vec<Trade>> = assert_ok!(serde_json::from_slice(trades.as_slice()));
	}

	#[test]
	fn test_summary_deserialization() {
		let summary = load_test_data(Call::Markets(Market::TwentyFourHourSummary(OneOrAllMarkets::One)));
		let _: Response<MarketSummary> = assert_ok!(serde_json::from_slice(summary.as_slice()));
	}

	#[test]
	fn test_all_summary_deserialization() {
		let summary = load_test_data(Call::Markets(Market::TwentyFourHourSummary(OneOrAllMarkets::All)));
		let _: Response<AllMarketSummaries> = assert_ok!(serde_json::from_slice(summary.as_slice()));
	}

	#[test]
	fn test_orderbook_deserialization() {
		let orderbook = load_test_data(Call::Markets(Market::Orderbook(OrderbookCall::Book)));
		let _: Response<Orderbook> = assert_ok!(serde_json::from_slice(orderbook.as_slice()));
	}

	#[test]
	fn test_orderbook_liquidity_deserialization() {
		let orderbook = load_test_data(Call::Markets(Market::Orderbook(OrderbookCall::Liquidity)));
		let _: Response<OrderbookLiquidity> = assert_ok!(serde_json::from_slice(orderbook.as_slice()));
	}

	#[test]
	fn test_orderbook_calculator_deserialization() {
		let orderbook = load_test_data(Call::Markets(Market::Orderbook(OrderbookCall::Calculator)));
		let _: Response<OrderbookCalculator> = assert_ok!(serde_json::from_slice(orderbook.as_slice()));
	}

	// Markets -> OHLC
	#[test]
	fn test_ohlc_deserialization() {
		let ohlc = load_test_data(Call::Markets(Market::OHLC));
		let _: Response<PeriodMap> = assert_ok!(serde_json::from_slice(ohlc.as_slice()));
	}
}
