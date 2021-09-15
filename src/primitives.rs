// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of polkadot-rewards.

// polkadot-rewards is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// polkadot-rewards is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with polkadot-rewards.  If not, see <http://www.gnu.org/licenses/>.

use chrono::NaiveDate;
use cli_table::Table;
use serde::{Deserialize, Serialize};
use std::{
	collections::{BTreeSet, HashMap},
	fmt,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
	code: usize,
	message: String,
	ttl: Option<usize>,
	generated_at: Option<usize>,
	data: T,
}

impl<T> ApiResponse<T> {
	pub fn consume(self) -> T {
		self.data
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List<T> {
	pub count: usize,
	pub list: Option<Vec<T>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
	pub market_data: MarketData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarketData {
	pub current_price: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reward {
	pub event_index: String,
	pub block_num: u64,
	pub extrinsic_idx: u64,
	pub module_id: String,
	pub event_id: String,
	pub params: serde_json::Value, // leaving this as general type because we don't need it and i'm lazy
	pub extrinsic_hash: String,
	pub event_idx: usize,
	pub amount: String,
	pub block_timestamp: usize,
}

#[derive(Debug)]
pub struct RewardEntry {
	pub block_nums: BTreeSet<u64>,
	pub day: NaiveDate,
	pub amount: u128,
}

#[derive(Debug)]
pub struct SeparatedRewardEntry {
	pub block_num: u64,
	pub day: NaiveDate,
	pub amount: u128,
}

// "block_num,block_time,amount_dot,price_usd,price_time"
#[derive(Debug, Serialize, Table, Clone)]
pub struct CsvRecord {
	#[table(title = "Date")]
	pub date: String,
	#[table(title = "Amount")]
	pub amount: f64,
	#[table(title = "Blocks")]
	pub block_nums: String,
	#[table(title = "Price")]
	pub price: OptionalPrice,
}

#[derive(Debug, Clone, Serialize)]
pub struct OptionalPrice(Option<f64>);
impl OptionalPrice {
	pub fn new(price: Option<f64>) -> Self {
		Self(price)
	}
}
impl fmt::Display for OptionalPrice {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(p) = self.0 {
			write!(f, "{}", p)
		} else {
			write!(f, "")
		}
	}
}

impl From<&Option<f64>> for OptionalPrice {
	fn from(price: &Option<f64>) -> OptionalPrice {
		OptionalPrice(*price)
	}
}
