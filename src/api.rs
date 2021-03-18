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

//! Wrapper around calls to Subscans API

use crate::{
	cli::{App, Network},
	primitives::{ApiResponse, List, Price, Reward, RewardEntry},
};
use anyhow::{Context, Error};
use chrono::naive::NaiveDateTime;
use indicatif::ProgressBar;
use std::{collections::BTreeMap, convert::TryInto};

const POLKADOT_ENDPOINT: &str = "https://polkadot.subscan.io/api/";
const KUSAMA_ENDPOINT: &str = "https://kusama.subscan.io/api/";
const PRICE_ENDPOINT: &str = "https://api.coingecko.com/api/v3";
const REWARD_SLASH: &str = "scan/account/reward_slash";

fn get_endpoint(network: &Network, end: &str) -> String {
	match network {
		Network::Polkadot => format!("{}{}", POLKADOT_ENDPOINT, end),
		Network::Kusama => format!("{}{}", KUSAMA_ENDPOINT, end),
	}
}

fn price_endpoint(network: &Network, timestamp: usize) -> String {
	format!(
		"{}/coins/{}/history?date={}",
		PRICE_ENDPOINT,
		network.id(),
		NaiveDateTime::from_timestamp(timestamp.try_into().unwrap(), 0).format("%d-%m-%Y"),
	)
}

// TODO: Rate limit these requests so we don't end up trying to DoS subscan.

/// Wraps the subscan API to make things easy
pub struct Api<'a> {
	app: &'a App,
	progress: Option<&'a ProgressBar>,
	agent: ureq::Agent,
}

impl<'a> Api<'a> {
	/// instantiate a new instance of the subscan API
	pub fn new(app: &'a App, progress: Option<&'a ProgressBar>) -> Self {
		let agent = ureq::builder().build();

		Self { app, progress, agent }
	}

	/// get a price at a point in time from subscan.
	///
	/// `time`: UNIX timestamp of the time to query (UTC)
	fn price(&self, time: usize) -> Result<Price, Error> {
		let req = self.agent.get(&price_endpoint(&self.app.network, time));

		let price: Price = req.send_bytes(&[])?.into_json()?;
		Ok(price)
	}

	/// Get rewards from a specific page of subscan API
	///
	/// `page`: Which page to query
	/// `count`: How many to return in one request. There's some upper limit on this, probably something like 100
	fn rewards(&self, page: usize, count: usize) -> Result<List<Reward>, Error> {
		let req = self.agent.post(&get_endpoint(&self.app.network, REWARD_SLASH));

		let rewards = req
			.set("Content-Type", "application/json")
			.send_json(ureq::json!({
				"address": self.app.address.as_str(),
				"page": page,
				"row": count
			}))
			.with_context(|| {
				format!("Failed to fetch reward for address={} page={} count={}", self.app.address, page, count,)
			})?
			.into_string()?;
		let rewards: ApiResponse<List<Reward>> =
			serde_json::from_str(&rewards).with_context(|| format!("Failed to decode response: {}", rewards))?;
		Ok(rewards.consume())
	}
	/*
	fn historical_price(&self, from: String, to: String) {
		todo!()
	}
	*/
	/// Fetch all the rewards starting from some point in time, and ending at another
	///
	/// `from`: UNIX timestamp at which to begin returning rewards
	/// `to`: UNIX timestamp at which to end returning rewards
	pub fn fetch_all_rewards(&self, from: usize, to: usize) -> Result<Vec<RewardEntry>, Error> {
		self.progress.map(|r| r.reset());
		// get the first page only to get the count (query only one item)
		let total_pages = self.rewards(0, 1).context("Failed to fetch initial reward page")?.count / 100;

		self.progress.map(|p| p.set_message("Fetching Rewards"));
		self.progress.map(|p| p.set_length(total_pages as u64));

		let rewards: Vec<Reward> = (0..total_pages)
			.filter_map(|i| {
				self.progress.map(|p| p.inc(1));
				// subscan allows 10 requests per second
				std::thread::sleep(std::time::Duration::from_millis(100));
				self.rewards(i, 100)
					.with_context(|| format!("Failed to fetch page {} of {}", i, total_pages))
					.unwrap()
					.list
			})
			.flatten()
			.filter(|r| (r.block_timestamp >= from) && (r.block_timestamp <= to))
			.collect();

		// TODO: this is kind of cheating but it's easier than trying to query just what we need
		self.progress.map(|p| p.finish());

		// merge all entries from the same day
		let mut merged = BTreeMap::new();
		for reward in rewards {
			let day =
				NaiveDateTime::from_timestamp(reward.block_timestamp.try_into()?, 0).format("%Y-%m-%d").to_string();
			let amount: u128 = reward.amount.parse()?;
			let value = RewardEntry { block_num: reward.block_num, timestamp: reward.block_timestamp, amount };
			merged.entry(day).or_insert(value).amount += amount;
		}

		Ok(merged.into_iter().map(|(_k, v)| v).collect())
	}

	/// Returns a vector of prices corresponding to the passed-in vector of Rewards.
	pub fn fetch_prices(&self, rewards: &[RewardEntry]) -> Result<Vec<Price>, Error> {
		self.progress.map(|p| p.reset());
		self.progress.map(|p| p.set_length(rewards.len() as u64));
		self.progress.map(|p| p.set_message("Fetching Price Data"));
		let mut prices = Vec::new();
		for r in rewards.iter() {
			self.progress.map(|p| p.inc(1));
			// coingecko allows 100 requests per minute
			std::thread::sleep(std::time::Duration::from_millis(600));
			prices.push(self.price(r.timestamp)?)
		}
		self.progress.map(|p| p.finish_with_message("Prices Fetched"));
		Ok(prices)
	}
}
