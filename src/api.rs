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
	primitives::{ApiResponse, List, Price, Reward, RewardEntry, SeparatedRewardEntry},
};
use anyhow::{anyhow, Context, Error};
use chrono::{naive::NaiveDateTime, NaiveDate};
use indicatif::ProgressBar;
use std::{
	collections::{BTreeMap, BTreeSet},
	convert::TryInto,
};

const POLKADOT_ENDPOINT: &str = "https://polkadot.api.subscan.io/api/";
const KUSAMA_ENDPOINT: &str = "https://kusama.api.subscan.io/api/";
const MOONRIVER_ENDPOINT: &str = "https://moonriver.api.subscan.io/api/";
const MOONBEAM_ENDPOINT: &str = "https://moonbeam.api.subscan.io/api/";
const ASTAR_ENDPOINT: &str = "https://astar.api.subscan.io/api/";
const ALEPH_ENDPOINT: &str = "https://alephzero.api.subscan.io/api/";

const PRICE_ENDPOINT: &str = "https://api.coingecko.com/api/v3";
const REWARD_SLASH: &str = "scan/account/reward_slash";

fn get_endpoint(network: &Network, end: &str) -> String {
	match network {
		Network::Polkadot => format!("{}{}", POLKADOT_ENDPOINT, end),
		Network::Kusama => format!("{}{}", KUSAMA_ENDPOINT, end),
		Network::Moonriver => format!("{}{}", MOONRIVER_ENDPOINT, end),
		Network::Moonbeam => format!("{}{}", MOONBEAM_ENDPOINT, end),
		Network::Astar => format!("{}{}", ASTAR_ENDPOINT, end),
		Network::Aleph => format!("{}{}", ALEPH_ENDPOINT, end),
	}
}

fn price_endpoint(network: &Network, day: NaiveDate) -> String {
	format!("{}/coins/{}/history?date={}", PRICE_ENDPOINT, network.id(), day.format("%d-%m-%Y"),)
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
		let agent = ureq::builder().user_agent(&app.user).build();

		Self { app, progress, agent }
	}

	/// get a price at a point in time from subscan.
	///
	/// `time`: UNIX timestamp of the time to query (UTC)
	fn price(&self, day: NaiveDate) -> Result<Price, Error> {
		let req = self.agent.get(&price_endpoint(&self.app.network, day));

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
			.set("X-API-Key", &self.app.api_key)
			.send_json(ureq::json!({
				"address": self.app.address.as_str(),
				"page": page,
				"row": count
			}))
			.with_context(|| {
				format!("Failed to fetch reward for address={} page={} row={}", self.app.address, page, count,)
			})?
			.into_string()?;
		let rewards: ApiResponse<List<Reward>> =
			serde_json::from_str(&rewards).with_context(|| format!("Failed to decode response: {}", rewards))?;
		Ok(rewards.consume())
	}

	/// Fetch all rewardsstarting from some point in time and ending at another.
	fn fetch_rewards(&self) -> Result<Vec<Reward>, Error> {
		const PAGE_SIZE: usize = 100;

		self.progress.map(|r| r.reset());
		self.progress.map(|p| p.set_message("Fetching Rewards"));
		self.progress.map(|r| r.tick());

		let page_estimate = {
			let num_entries = self.rewards(0, 1).context("Failed to fetch initial reward page")?.count;
			let full_pages = num_entries / PAGE_SIZE;
			if num_entries % PAGE_SIZE == 0 {
				full_pages
			} else {
				full_pages + 1
			}
		};

		self.progress.map(|p| p.set_message("Fetching Rewards"));
		self.progress.map(|p| p.set_length(page_estimate.try_into().unwrap()));
		self.progress.map(|r| r.tick());

		let rewards: Vec<Reward> = (0..)
			.map(|i| {
				self.progress.map(|p| p.inc(1));
				// subscan allows 10 requests per second
				std::thread::sleep(std::time::Duration::from_millis(300));
				self.rewards(i, PAGE_SIZE).with_context(|| format!("Failed to fetch page {}", i)).unwrap().list
			})
			.take_while(|list| list.is_some())
			.flatten()
			.flatten()
			.filter(|r| {
				let timestamp = NaiveDateTime::from_timestamp(r.block_timestamp.try_into().unwrap(), 0);
				let from = if let Some(from) = self.app.from { timestamp >= from } else { true };
				let to = if let Some(to) = self.app.to { timestamp <= to } else { true };
				from && to
			})
			.collect();

		self.progress.map(|p| p.finish());
		Ok(rewards)
	}

	/// Fetch all rewards, joining blocks with rewards on the same day
	pub fn fetch_all_rewards(&self) -> Result<Vec<RewardEntry>, Error> {
		let rewards = self.fetch_rewards()?;
		// TODO: this is kind of cheating but it's easier than trying to query just what we need
		self.progress.map(|p| p.finish());

		// merge all entries from the same day
		let mut merged = BTreeMap::new();
		for reward in rewards {
			let day = NaiveDateTime::from_timestamp(reward.block_timestamp.try_into()?, 0).date();
			let amount: u128 = reward.amount.parse()?;
			let value = RewardEntry {
				block_nums: {
					let mut blocks = BTreeSet::new();
					blocks.insert(reward.block_num);
					blocks
				},
				day,
				amount,
			};
			merged
				.entry(day)
				.and_modify(|e: &mut RewardEntry| {
					e.block_nums.insert(reward.block_num);
					e.amount += amount;
				})
				.or_insert(value);
		}

		Ok(merged.into_iter().map(|(_k, v)| v).rev().collect())
	}

	pub fn fetch_all_rewards_separated(&self) -> Result<Vec<SeparatedRewardEntry>, Error> {
		let mut separated_rewards = Vec::new();
		let rewards = self.fetch_rewards()?;
		for reward in rewards {
			let date = NaiveDateTime::from_timestamp(reward.block_timestamp.try_into()?, 0);
			let amount: u128 = reward.amount.parse()?;
			let value =
				SeparatedRewardEntry { block_num: reward.block_num, amount, day: date.date(), time: date.time() };
			separated_rewards.push(value);
		}
		Ok(separated_rewards)
	}

	/// Returns a vector of prices corresponding to the passed-in vector of Rewards.
	pub fn fetch_prices(&self, dates: &[NaiveDate]) -> Result<Vec<f64>, Error> {
		self.progress.map(|p| p.reset());
		self.progress.map(|p| p.set_message("Fetching Price Data"));
		self.progress.map(|p| p.set_length(dates.len().try_into().unwrap()));
		self.progress.map(|r| r.tick());
		let mut prices = Vec::with_capacity(dates.len());
		for day in dates {
			self.progress.map(|p| p.inc(1));
			// coingecko allows 100 requests per minute
			// it seems to be a bit oversensitive. We therefore restrain ourselfs
			// to 60 requests a minute.
			std::thread::sleep(std::time::Duration::from_millis(1000));
			let result = self.price(*day)?;
			let price = result.market_data.current_price.get(&self.app.currency).ok_or_else(|| {
				anyhow!(
					"Specified fiat currency '{}' not supported: {:#?}",
					self.app.currency,
					result.market_data.current_price.keys(),
				)
			})?;
			prices.push(*price);
		}
		self.progress.map(|p| p.finish_with_message("Prices Fetched"));
		Ok(prices)
	}
}
