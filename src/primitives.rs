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

use miniserde::{json, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T: Serialize + Deserialize> {
    code: usize,
    message: String,
    ttl: Option<usize>,
    generated_at: Option<usize>,
    data: T,
}

impl<T: Serialize + Deserialize> ApiResponse<T> {
    pub fn consume(self) -> T {
        self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List<T: Serialize + Deserialize> {
    count: usize,
    list: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    price: String,
    time: usize,
    height: usize,
    records: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    price: String,
    height: usize,
    time: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reward {
    event_index: String,
    block_num: usize,
    extrinsic_idx: usize,
    module_id: String,
    event_id: String,
    params: json::Value, // leaving this as general type because we don't need it and i'm lazy
    extrinsic_hash: String,
    event_idx: usize,
    amount: String,
    block_timestamp: usize,
    slash_kton: String,
}
