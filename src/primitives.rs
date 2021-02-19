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

use serde::{Deserialize, Serialize};

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
    pub list: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub price: String,
    pub time: usize,
    pub height: usize,
    pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    price: String,
    height: usize,
    time: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reward {
    pub event_index: String,
    pub block_num: usize,
    pub extrinsic_idx: usize,
    pub module_id: String,
    pub event_id: String,
    pub params: serde_json::Value, // leaving this as general type because we don't need it and i'm lazy
    pub extrinsic_hash: String,
    pub event_idx: usize,
    pub amount: String,
    pub block_timestamp: usize,
    pub slash_kton: String,
}

// "block_num,block_time,amount_dot,price_usd,price_time"
#[derive(Debug, Serialize)]
pub struct CsvRecord {
    pub block_num: usize,
    pub block_time: String,
    pub amount: f64,
    pub price: f64,
    pub time: String,
}
