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
    primitives::{ApiResponse, List, Price, Reward},
};
use anyhow::Error;
use indicatif::ProgressBar;

const POLKADOT_ENDPOINT: &str = "https://polkadot.subscan.io/api/";
const KUSAMA_ENDPOINT: &str = "https://kusama.subscan.io/api/";
const PRICE: &str = "open/price";
const REWARD_SLASH: &str = "scan/account/reward_slash";

fn get_endpoint(network: &Network, end: &str) -> String {
    match network {
        Network::Polkadot => format!("{}{}", POLKADOT_ENDPOINT, end),
        Network::Kusama => format!("{}{}", KUSAMA_ENDPOINT, end),
    }
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

        Self {
            app,
            progress,
            agent,
        }
    }

    /// get a price at a point in time from subscan.
    ///
    /// `time`: UNIX timestamp of the time to query (UTC)
    fn price(&self, time: usize) -> Result<Price, Error> {
        let req = self.agent.post(&get_endpoint(&self.app.network, PRICE));

        let price: ApiResponse<Price> = req
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({ "time": time }))?
            .into_json()?;
        Ok(price.consume())
    }

    /// Get rewards from a specific page of subscan API
    ///
    /// `page`: Which page to query
    /// `count`: How many to return in one request. There's some upper limit on this, probably something like 100
    fn rewards(&self, page: usize, count: usize) -> Result<List<Reward>, Error> {
        let req = self
            .agent
            .post(&get_endpoint(&self.app.network, REWARD_SLASH));

        let rewards = req
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({
                "address": self.app.address.as_str(),
                "page": page,
                "row": count
            }))?
            .into_string()?;
        let rewards: ApiResponse<List<Reward>> = serde_json::from_str(&rewards)?;
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
    pub fn fetch_all_rewards(&self, from: usize, to: usize) -> Result<Vec<Reward>, Error> {
        if let Some(r) = self.progress {
            r.reset()
        }
        let mut rewards = Vec::new();
        // first, get rewards from the first page
        let reward = self.rewards(0, 10)?;
        let total_pages = reward.count / 10;
        rewards.extend(reward.list.into_iter());

        self.progress.map(|p| p.set_message("Fetching Rewards"));
        self.progress.map(|p| p.set_length(total_pages as u64));

        for i in 1..=total_pages {
            self.progress.map(|p| p.inc(1));
            // rate limited
            std::thread::sleep(std::time::Duration::from_millis(35));
            rewards.extend(self.rewards(i, 10)?.list.into_iter());
        }
        // TODO: this is kind of cheating but it's easier than trying to query just what we need
        self.progress
            .map(|p| p.finish_with_message(&format!("Total Rewards Received: {}", rewards.len())));

        Ok(rewards
            .into_iter()
            .filter(|r| (r.block_timestamp >= from) && (r.block_timestamp <= to))
            .collect())
    }

    /// Returns a vector of prices corresponding to the passed-in vector of Rewards.
    pub fn fetch_prices(&self, rewards: &[Reward]) -> Result<Vec<Price>, Error> {
        self.progress.map(|p| p.reset());
        self.progress.map(|p| p.set_length(rewards.len() as u64));
        self.progress.map(|p| p.set_message("Fetching Price Data"));
        let mut prices = Vec::new();
        for r in rewards.iter() {
            self.progress.map(|p| p.inc(1));
            // we're rate limited at 10 req/s
            std::thread::sleep(std::time::Duration::from_millis(35));
            prices.push(self.price(r.block_timestamp)?)
        }
        self.progress
            .map(|p| p.finish_with_message(&format!("Prices Fetched")));
        Ok(prices)
    }
}
