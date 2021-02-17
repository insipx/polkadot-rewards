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

use crate::{api::Api, primitives::CsvRecord};
use anyhow::{bail, Error};
use argh::FromArgs;
use chrono::{
    naive::NaiveDateTime,
    offset::{TimeZone, Utc},
};
use std::{convert::TryInto, fs::File, io, path::PathBuf, str::FromStr};

#[derive(FromArgs, PartialEq, Debug)]
/// Polkadot Staking Rewards CLI-App
pub struct App {
    #[argh(option, from_str_fn(date_from_string), short = 'f')]
    /// define the when to start crawling for staking rewards
    pub from: chrono::NaiveDateTime,
    /// define when to stop crawling for staking rewards. Defaults to current time.
    #[argh(option, default = "Utc::now()", short = 't')]
    pub to: chrono::DateTime<Utc>,
    /// the network to crawl for rewards. One of: Polkadot, Kusama
    #[argh(option, default = "Network::Polkadot", short = 'n')]
    pub network: Network,
    /// network-formatted Address to get staking rewards for
    #[argh(option, short = 'a')]
    pub address: String,
    /// directory to output completed CSV to
    #[argh(option, default = "default_file_location()", short = 'p')]
    folder: PathBuf,
    /// output the CSV file to STDOUT. Disables creating a new file.
    #[argh(switch, short = 's')]
    stdout: bool,
}

fn default_file_location() -> PathBuf {
    match std::env::current_dir() {
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(1);
        }
        Ok(p) => p,
    }
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

impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::Kusama => "ksm".to_string(),
            Network::Polkadot => "dot".to_string(),
        }
    }
}

pub fn app() -> Result<(), Error> {
    let mut app: App = argh::from_env();
    let api = Api::new(&app);
    let rewards =
        api.fetch_all_rewards(app.from.timestamp() as usize, app.to.timestamp() as usize)?;
    let prices = api.fetch_prices(&rewards)?;

    app.folder.push(construct_file_name(&app));
    app.folder.set_extension("csv");

    let mut wtr = Output::new(&app)?;

    for (reward, price) in rewards.iter().zip(prices.iter()) {
        wtr.serialize(CsvRecord {
            block_num: reward.block_num,
            block_time: Utc.timestamp(reward.block_timestamp.try_into()?, 0),
            amount: amount_to_network(&app.network, &reward.amount)?,
            price: f64::from_str(&price.price)?,
            time: Utc.timestamp(price.time.try_into()?, 0),
        })?;
    }
    Ok(())
}

fn amount_to_network(network: &Network, amount: &str) -> Result<f64, Error> {
    match network {
        Network::Polkadot => Ok(f64::from_str(amount)? / (10000000000f64)),
        Network::Kusama => Ok(f64::from_str(amount)? / (10000000000000f64)),
    }
}

// constructs a file name in the format: `dot-address-from_date-to_date-rewards.csv`
fn construct_file_name(app: &App) -> String {
    format!(
        "{}-{}-{}-{}-rewards",
        app.network.to_string(),
        &app.address,
        app.from.to_string(),
        app.to.to_string()
    )
}

enum Output {
    FileOut(csv::Writer<File>),
    StdOut(csv::Writer<std::io::Stdout>),
}

impl Output {
    fn new(app: &App) -> Result<Self, Error> {
        if app.stdout {
            Ok(Output::StdOut(csv::Writer::from_writer(io::stdout())))
        } else {
            let file = File::create(&app.folder)?;
            Ok(Output::FileOut(csv::Writer::from_writer(file)))
        }
    }

    fn serialize<T: serde::Serialize>(&mut self, val: T) -> Result<(), Error> {
        match self {
            Output::FileOut(f) => f.serialize(val)?,
            Output::StdOut(s) => s.serialize(val)?,
        };
        Ok(())
    }
}
