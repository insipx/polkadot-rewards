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

use crate::api::Api;
use anyhow::{bail, Error};
use argh::FromArgs;
use chrono::{naive::NaiveDateTime, offset::Utc};
use std::str::FromStr;

#[derive(FromArgs, PartialEq, Debug)]
/// Polkadot Staking Rewards CLI-App
pub struct App {
    #[argh(option, from_str_fn(date_from_string))]
    /// define the when to start crawling for staking rewards
    pub from: chrono::NaiveDateTime,
    /// define when to stop crawling for staking rewards. Defaults to current time.
    #[argh(option, default = "Utc::now()")]
    pub to: chrono::DateTime<Utc>,
    /// the network to crawl for rewards. One of: Polkadot, Kusama
    #[argh(option, default = "Network::Polkadot")]
    pub network: Network,
    /// network-Formatted Address to get staking rewards for
    #[argh(option)]
    pub address: String,
}

// we don't return an anyhow::Error here because `argh` macro expects error type to be a `String`
pub fn date_from_string(value: &str) -> Result<chrono::NaiveDateTime, String> {
    let time = match NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string()),
    };
    let time = time?;
    println!("{:?}", time.timestamp());
    Ok(time)
}

#[derive(PartialEq, Debug)]
pub enum Network {
    /// The Polkadot Network
    Polkadot,
    /// The Kusama Network
    Kusama,
}

impl FromStr for Network {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "polkadot" | "dot" => Ok(Network::Polkadot),
            "kusama" | "ksm" => Ok(Network::Kusama),
            _ => bail!("Network must be one of: 'kusama', 'polkadot', 'dot', 'ksm'"),
        }
    }
}

pub fn app() -> Result<(), Error> {
    let app: App = argh::from_env();
    let api = Api::new(&app);
    println!("FROM: {}", app.from.timestamp());
    let rewards =
        api.fetch_all_rewards(app.from.timestamp() as usize, app.to.timestamp() as usize)?;
    let prices = api.fetch_prices(&rewards)?;

    println!("Rewards: {}", miniserde::json::to_string(&rewards));
    println!("Prices: {}", miniserde::json::to_string(&prices));
    Ok(())
}
