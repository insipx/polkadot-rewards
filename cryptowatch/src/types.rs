use chrono::naive::NaiveDateTime;
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
	result: T,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum Period<T> {
	P60(T),
	P180(T),
	P300(T),
	P900(T),
	P1800(T),
	P3600(T),
	P7200(T),
	P14400(T),
	P21600(T),
	P43200(T),
	P86400(T),
	P259200(T),
	P604800(T),
	P604800Monday(T),
}

#[derive(Serialize, Deserialize)]
pub struct OHLC {
	close_time: NaiveDateTime,
	open_price: f64,
	high_price: f64,
	low_price: f64,
	close_price: f64,
	volume: f64,
	quote_volume: f64,
}
