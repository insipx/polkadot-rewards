//! Implementation of Market types

use crate::{error::Error, types::*};
use serde::de::{self, Visitor};
use std::{collections::HashMap, str::FromStr};

struct PriceMapVisitor;
impl<'de> Visitor<'de> for PriceMapVisitor {
	type Value = PriceMap;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a map representing a market, exchange, pair with the key as the price")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: de::MapAccess<'de>,
	{
		let mut price_map = HashMap::new();
		while let Some((key, value)) = map.next_entry::<String, f64>()? {
			let mut key = key.split(':');
			let investment_vehicle = key
				.next()
				.expect("investment vehicle")
				.try_into()
				.expect("`InvestmentVehicle` should be in correct format");
			let exchange = key.next().expect("exchange").try_into().map_err(de::Error::custom)?;
			let pair = key.next().expect("pair").to_string();
			let price = value;
			price_map.insert((investment_vehicle, exchange, pair), price);
		}
		Ok(PriceMap(price_map))
	}
}

impl<'de> de::Deserialize<'de> for PriceMap {
	fn deserialize<D>(deserializer: D) -> Result<PriceMap, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		deserializer.deserialize_map(PriceMapVisitor)
	}
}

struct MarketSummaryVisitor;
impl<'de> Visitor<'de> for MarketSummaryVisitor {
	type Value = AllMarketSummaries;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a map representing market summaries")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: de::MapAccess<'de>,
	{
		let mut market_summaries = HashMap::new();
		while let Some((key, value)) = map.next_entry::<String, MarketSummary>()? {
			let mut key = key.split(':');
			let exchange = key.next().expect("exchange").try_into().map_err(de::Error::custom)?;
			let pair: Pair = key.next().expect("pair").into();
			market_summaries.insert((exchange, pair), value);
		}
		Ok(AllMarketSummaries { inner: market_summaries })
	}
}

impl<'de> de::Deserialize<'de> for AllMarketSummaries {
	fn deserialize<D>(deserializer: D) -> Result<AllMarketSummaries, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		deserializer.deserialize_map(MarketSummaryVisitor)
	}
}

struct OfferVisitor;
impl<'de> Visitor<'de> for OfferVisitor {
	type Value = Offer;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a map representing an offer")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Offer, A::Error>
	where
		A: de::SeqAccess<'de>,
	{
		let price = seq.next_element::<f64>()?.expect("price must not be empty");
		let amount = seq.next_element::<f64>()?.expect("amount must not be empty");
		Ok(Offer { price, amount })
	}
}

impl<'de> de::Deserialize<'de> for Offer {
	fn deserialize<D>(deserializer: D) -> Result<Offer, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		deserializer.deserialize_seq(OfferVisitor)
	}
}

struct F64Visitor;
impl<'de> Visitor<'de> for F64Visitor {
	type Value = F64;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("a string representing a floating point number")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		Ok(F64(v.parse().map_err(de::Error::custom)?))
	}
}

impl<'de> de::Deserialize<'de> for F64 {
	fn deserialize<D>(deserializer: D) -> Result<F64, D::Error>
	where
		D: de::Deserializer<'de>,
	{
		deserializer.deserialize_str(F64Visitor)
	}
}

impl std::ops::Deref for F64 {
	type Target = f64;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

// -------------------------- Conversions

impl FromStr for F64 {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let basis: f64 = s
			.parse()
			.expect("the string for a basis should always be a floating point integer.");
		Ok(F64(basis))
	}
}

impl ToString for InvestmentVehicle {
	fn to_string(&self) -> String {
		match self {
			InvestmentVehicle::Market => "market",
			InvestmentVehicle::Index => "index",
		}
		.to_string()
	}
}

impl FromStr for InvestmentVehicle {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"market" => Ok(InvestmentVehicle::Market),
			"index" => Ok(InvestmentVehicle::Index),
			_ => Err(Error::InvalidString(s.to_string())),
		}
	}
}

impl TryFrom<String> for InvestmentVehicle {
	type Error = Error;

	fn try_from(s: String) -> Result<Self, Self::Error> {
		InvestmentVehicle::from_str(s.as_str())
	}
}

impl<'a> TryFrom<&'a str> for InvestmentVehicle {
	type Error = Error;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		InvestmentVehicle::from_str(s)
	}
}
