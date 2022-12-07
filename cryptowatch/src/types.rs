use chrono::{serde::ts_seconds, DateTime, Utc};
use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

/// The general REST API "Response" type.
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
	result: T,
	allowance: ApiAllowance<'static>,
}

impl<T> Response<T> {
	/// unpack the `[Response]` yeilding its associated body, `T`.
	pub fn unpack(self) -> T {
		self.result
	}
}

/// Information about the amount of credits left before a user reaches their rate-limit.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiAllowance<'a> {
	cost: f64,
	remaining: f64,
	upgrade: Cow<'a, str>,
}

impl<'a> std::fmt::Display for ApiAllowance<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(
			f,
			"This api call cost {} and you have {} credits remaining. {}",
			self.cost, self.remaining, self.upgrade
		)
	}
}

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

/// An asset/asset pairing.
#[derive(Serialize, Deserialize, Debug)]
pub struct Pair<'a> {
	id: u64,
	#[serde(borrow)]
	symbol: Cow<'a, str>,
	#[serde(borrow)]
	base: Asset<'a>,
	#[serde(borrow)]
	quote: Asset<'a>,
	#[serde(borrow)]
	route: Cow<'a, str>,
}

/// Maping of Period to the [`OHLC`] data for that length of time
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
	use std::path::PathBuf;

	// TODO: Return an impl reader and accept an enum for type of response.
	fn load_test_data() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
		let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		let (mut assets, mut pairs, mut ohlc) = (manifest_dir.clone(), manifest_dir.clone(), manifest_dir.clone());
		assets.push("data/assets.json");
		pairs.push("data/pairs.json");
		ohlc.push("data/ohlc.json");

		(std::fs::read(assets).unwrap(), std::fs::read(pairs).unwrap(), std::fs::read(ohlc).unwrap())
	}

	#[test]
	fn test_asset_deserialization() {
		let (assets, _, _) = load_test_data();
		let _: Response<Vec<Asset>> = serde_json::from_slice(assets.as_slice()).unwrap();
	}

	#[test]
	fn test_pair_deserialization() {
		let (_, pairs, _) = load_test_data();
		let _: Response<Vec<Pair>> = serde_json::from_slice(pairs.as_slice()).unwrap();
	}

	#[test]
	fn test_ohlc_deserialization() {
		let (_, _, ohlc) = load_test_data();
		let _: Response<PeriodMap> = serde_json::from_slice(ohlc.as_slice()).unwrap();
	}
}
