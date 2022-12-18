//! Types representing Markets.
use super::*;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::collections::HashMap;

/// An asset describing a market
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Market {
	id: u64,
	exchange: Exchange,
	pair: String,
	active: bool,
	#[serde(flatten)]
	route: SingleOrMultipleRoutes,
}

/// How the investment was made. Either through a Market or Index.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvestmentVehicle {
	Market,
	Index,
}

/// A mapping of investment vehicle, exchange, and pair to a price.
#[derive(Debug, PartialEq)]
pub struct PriceMap(pub HashMap<(InvestmentVehicle, Exchange, String), f64>);

/// A trade that occured at some time (UTC).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Trade {
	id: u64,
	#[serde(with = "ts_seconds")]
	timestamp: DateTime<Utc>,
	price: f64,
	amount: f64,
}

/// Summary of the market for one asset in a 24-hour time period
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarketSummary {
	/// Price of the asset
	price: PriceSummary,
	/// Volume of the base asset
	volume: f64,
	/// Volume of the quote asset
	#[serde(rename = "volumeQuote")]
	volume_quote: f64,
}

/// Summary of the price of an asset over a time period.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PriceSummary {
	last: f64,
	high: f64,
	low: f64,
	change: PriceChange,
}

/// percentage change in a price over a time period.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PriceChange {
	percentage: f64,
	absolute: f64,
}

/// Summaries of every market and every pair.
#[derive(Serialize, Debug)]
pub struct AllMarketSummaries {
	pub(crate) inner: HashMap<(Exchange, Pair), MarketSummary>,
}

/// An individual offer to either sell or buy an `amount` of an asset at a `price`.
#[derive(Debug, Serialize, PartialEq)]
pub struct Offer {
	pub(crate) price: f64,
	pub(crate) amount: f64,
}

/// The order book for a given market
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[serde(transparent)]
pub(crate) struct F64(pub f64);

/// Sums of the base and quote assets at different point levels.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
	N25 = 25u64,
	N50 = 50u64,
	N75 = 75u64,
	N100 = 100u64,
	N150 = 150u64,
	N200 = 200u64,
	N250 = 250u64,
	N300 = 300u64,
	N400 = 400u64,
	N500 = 500u64,
}

/// Liquidity sums at several basis point levels in the order book.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderbookLiquidity {
	/// Asks for an asset. The lowest price a seller is willing to sell the asset.
	ask: LiquiditySums,
	/// Bids for an asset. The highest price a buyer is willing to buy the asset.
	bid: LiquiditySums,
}

/// Provides a live quote from the order book for a given buy & sell amount
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OrderbookCalculator {
	buy: OrderbookQuote,
	sell: OrderbookQuote,
}

/// Average price and deltas of a sell or buy
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum SpendOrReceive {
	#[serde(rename = "spend")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	Spend(f64),
	#[serde(rename = "receive")]
	#[serde(deserialize_with = "deserialize_number_from_string")]
	Receive(f64),
}

/// Mapping of Period to the [`OHLC`] data for that length of time
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct PeriodMap(HashMap<Period, Vec<OHLC>>);

/// The candle length ("Period") in seconds. Ranges from 1 minute to 1 week.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Period {
	#[serde(rename = "60")]
	N60,
	#[serde(rename = "180")]
	N180,
	#[serde(rename = "300")]
	N300,
	#[serde(rename = "900")]
	N900,
	#[serde(rename = "1800")]
	N1800,
	#[serde(rename = "3600")]
	N3600,
	#[serde(rename = "7200")]
	N7200,
	#[serde(rename = "14400")]
	N14400,
	#[serde(rename = "21600")]
	N21600,
	#[serde(rename = "43200")]
	N43200,
	#[serde(rename = "86400")]
	N86400,
	#[serde(rename = "259200")]
	N259200,
	#[serde(rename = "604800")]
	N604800,
	#[serde(rename = "604800_Monday")]
	N604800Monday,
}

/// "OHLC" data for a period of time. "OHLC" stands for "Open-High-Low-Close"
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
	use crate::tests::{data_prelude::*, prelude::*};

	#[tokio::test]
	async fn test_market_list_deserialization() {
		let list = test_data(Call::Markets(MarketQuery::List)).await;
		let _: Response<Vec<Market>> = assert_ok!(serde_json::from_slice(list.as_slice()));
	}

	#[tokio::test]
	async fn test_market_details_deserialization() {
		let details = test_data(Call::Markets(MarketQuery::Details)).await;
		let _: Response<Market> = assert_ok!(serde_json::from_slice(details.as_slice()));
	}

	#[tokio::test]
	async fn test_market_price_deserialization() {
		let price = test_data(Call::Markets(MarketQuery::Price(OneOrAllMarkets::One))).await;
		let _: Response<Price> = assert_ok!(serde_json::from_slice(price.as_slice()));
	}

	#[tokio::test]
	async fn test_all_market_price_deserialization() {
		let price = test_data(Call::Markets(MarketQuery::Price(OneOrAllMarkets::All))).await;
		let _: Response<PriceMap> = assert_ok!(serde_json::from_slice(price.as_slice()));
	}

	#[tokio::test]
	async fn test_trades_deserialization() {
		let trades = test_data(Call::Markets(MarketQuery::Trades)).await;
		let _: Response<Vec<Trade>> = assert_ok!(serde_json::from_slice(trades.as_slice()));
	}

	#[tokio::test]
	async fn test_summary_deserialization() {
		let summary = test_data(Call::Markets(MarketQuery::TwentyFourHourSummary(OneOrAllMarkets::One))).await;
		let _: Response<MarketSummary> = assert_ok!(serde_json::from_slice(summary.as_slice()));
	}

	#[tokio::test]
	async fn test_all_summary_deserialization() {
		let summary = test_data(Call::Markets(MarketQuery::TwentyFourHourSummary(OneOrAllMarkets::All))).await;
		let _: Response<AllMarketSummaries> = assert_ok!(serde_json::from_slice(summary.as_slice()));
	}

	#[tokio::test]
	async fn test_orderbook_deserialization() {
		let orderbook = test_data(Call::Markets(MarketQuery::Orderbook(OrderbookCall::Book))).await;
		let _: Response<Orderbook> = assert_ok!(serde_json::from_slice(orderbook.as_slice()));
	}

	#[tokio::test]
	async fn test_orderbook_liquidity_deserialization() {
		let orderbook = test_data(Call::Markets(MarketQuery::Orderbook(OrderbookCall::Liquidity))).await;
		let _: Response<OrderbookLiquidity> = assert_ok!(serde_json::from_slice(orderbook.as_slice()));
	}

	#[tokio::test]
	async fn test_orderbook_calculator_deserialization() {
		let orderbook = test_data(Call::Markets(MarketQuery::Orderbook(OrderbookCall::Calculator))).await;
		let _: Response<OrderbookCalculator> = assert_ok!(serde_json::from_slice(orderbook.as_slice()));
	}

	// Markets -> OHLC
	#[tokio::test]
	async fn test_ohlc_deserialization() {
		let ohlc = test_data(Call::Markets(MarketQuery::Ohlc)).await;
		let _: Response<PeriodMap> = assert_ok!(serde_json::from_slice(ohlc.as_slice()));
	}
}
