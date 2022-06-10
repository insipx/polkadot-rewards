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

use crate::{
	api::Api,
	primitives::{CsvRecord, GroupedCsvRecord, Output, SeparatedCsvRecord},
};
use anyhow::{anyhow, bail, ensure, Context, Error};
use argh::FromArgs;
use chrono::{naive::NaiveDateTime, NaiveDate};
use env_logger::{Builder, Env};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use sp_arithmetic::{FixedPointNumber, FixedU128};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

const OUTPUT_DATE: &str = "%Y-%m-%d";
const OUTPUT_TIME: &str = "%H:%M:%S";

#[derive(FromArgs, PartialEq, Debug)]
/// Polkadot Staking Rewards CLI-App
pub struct App {
	#[argh(option, from_str_fn(date_from_string), short = 'f')]
	/// date to start crawling for staking rewards. Format: "YYY-MM-DD HH:MM:SS"
	pub from: Option<NaiveDateTime>,
	/// date to stop crawling for staking rewards. Format: "YYY-MM-DD HH:MM:SS"
	#[argh(option, from_str_fn(date_from_string), short = 't')]
	pub to: Option<NaiveDateTime>,
	/// network to crawl for rewards. One of: [Polkadot, Kusama, Moonriver, MOVR, KSM, DOT]
	#[argh(option, default = "Network::Polkadot", short = 'n')]
	pub network: Network,
	/// the fiat currency which should be used for prices
	#[argh(option, short = 'c')]
	pub currency: String,
	/// network-formatted address to get staking rewards for.
	#[argh(option, short = 'a')]
	pub address: String,
	/// change the user agent for HTTPS requests
	#[argh(option, short = 'u', default = "default_user_agent()")]
	pub user: String,
	/// date format to use in output CSV data. Uses rfc2822 by default.  EX: "%Y-%m-%d %H:%M:%S".
	#[argh(option, default = "OUTPUT_DATE.to_string()")]
	pub date_format: String,
	#[argh(option, default = "OUTPUT_TIME.to_string()")]
	/// time format to use with `--no-group` flag. Default "%H:%M:%S".
	pub time_format: String,
	/// directory to output completed CSV to.
	#[argh(option, default = "default_file_location()", short = 'p')]
	pub folder: PathBuf,
	/// output the CSV file to STDOUT. Disables creating a new file.
	#[argh(switch, short = 's')]
	pub stdout: bool,
	#[argh(switch)]
	/// do not gather price data
	pub no_price: bool,
	#[argh(switch)]
	/// do not group blocks by day. Give each block its own column. Adds additional exact UTC `time` column.
	pub no_group: bool,
	#[argh(switch)]
	/// preview the rewards in your terminal instead of outputting CSV format.
	pub preview: bool,
	/// get extra information about the program execution.
	#[argh(switch, short = 'v')]
	pub verbose: bool,
	/// the subscan api key, optionally provided.
	#[argh(option, short = 'k')]
	pub api_key: String,
}

fn default_user_agent() -> String {
	let version = env!("CARGO_PKG_VERSION");
	format!("polkadot-rewards/{}", version)
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
	Ok(time)
}

#[derive(PartialEq, Debug)]
pub enum Network {
	/// The Polkadot Network
	Polkadot,
	/// The Kusama Network
	Kusama,
	/// The Moonriver Network
	Moonriver,
	/// The Moonbeam Network
	Moonbeam,
	/// The Astar network
	Astar,
}

impl Network {
	pub fn id(&self) -> &'static str {
		match self {
			Self::Polkadot => "polkadot",
			Self::Kusama => "kusama",
			Self::Moonbeam => "moonbeam",
			Self::Moonriver => "moonriver",
			Self::Astar => "astar",
		}
	}

	fn amount_to_network(&self, amount: &u128) -> Result<f64, Error> {
		let denominator = match self {
			Self::Polkadot => 10u128.pow(10),
			Self::Kusama => 10u128.pow(12),
			Self::Moonriver => 10u128.pow(18),
			Self::Moonbeam => 10u128.pow(18),
			Self::Astar => 10u128.pow(18),
		};
		let frac = FixedU128::checked_from_rational(*amount, denominator)
			.ok_or_else(|| anyhow!("Amount '{}' overflowed FixedU128", amount))?
			.to_float();
		Ok(frac)
	}
}

impl FromStr for Network {
	type Err = Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"polkadot" | "dot" => Ok(Network::Polkadot),
			"kusama" | "ksm" => Ok(Network::Kusama),
			"moonriver" | "movr" => Ok(Network::Moonriver),
			"moonbeam" | "glmr" => Ok(Network::Moonbeam),
			"astar" | "astr" => Ok(Network::Astar),
			_ => bail!(
				"Network must be one of: 'kusama', 'polkadot', 'moonbeam', 'astar', 'moonriver', or their
				token abbreviations."
			),
		}
	}
}

pub fn app() -> Result<(), Error> {
	let mut app: App = argh::from_env();
	let progress = if app.verbose {
		Builder::from_env(Env::default().default_filter_or("info")).init();
		None
	} else {
		Some(construct_progress_bar())
	};
	let api = Api::new(&app, progress.as_ref());

	let rewards = if app.no_group {
		CsvRecord::Separated(create_separated_rewards(&api, &app)?)
	} else {
		CsvRecord::Grouped(create_grouped_rewards(&api, &app)?)
	};

	let file_name = construct_file_name(&app, rewards.to_date_rev(), rewards.to_date());
	app.folder.push(&file_name);
	app.folder.set_extension("csv");

	if !app.preview {
		let mut wtr = Output::new(&app).context("Failed to create output.")?;
		rewards.serialize(&mut wtr)?;
		if app.stdout {
			progress.map(|p| p.finish_with_message("Writing data to STDOUT"));
		} else {
			progress.map(move |p| p.finish_with_message(format!("Wrote data to file: {}", file_name)));
		}
	} else {
		cli_table::print_stdout(rewards.with_title())?;
		progress.as_ref().map(|p| p.finish_with_message("Wrote preview"));
		progress.as_ref().map(|p| p.finish_and_clear());
	}

	Ok(())
}

fn create_grouped_rewards(api: &Api, app: &App) -> Result<Vec<GroupedCsvRecord>, Error> {
	let rewards = api.fetch_all_rewards().context("Failed to fetch rewards.")?;
	let prices = if app.no_price {
		(0..rewards.len()).into_iter().map(|_| None).collect::<Vec<Option<_>>>()
	} else {
		let dates: Vec<NaiveDate> = rewards.iter().map(|r| r.day).collect();
		api.fetch_prices(dates.as_slice()).context("Failed to fetch prices.")?.into_iter().map(Some).collect::<Vec<_>>()
	};

	ensure!(!rewards.is_empty(), "No rewards found for specified account.");

	rewards
		.iter()
		.zip(&prices)
		.map(|(reward, price)| {
			Ok(GroupedCsvRecord {
				block_nums: reward.block_nums.iter().fold(String::new(), |acc, i| format!("{}+{}", acc, i))[1..]
					.to_string(),
				date: reward.day.format(&app.date_format).to_string(),
				amount: app.network.amount_to_network(&reward.amount)?,
				price: price.into(),
			})
		})
		.collect::<Result<_, Error>>()
}

fn create_separated_rewards(api: &Api, app: &App) -> Result<Vec<SeparatedCsvRecord>, Error> {
	let rewards = api.fetch_all_rewards_separated().context("Failed to fetch rewards.")?;

	let dates = rewards.iter().map(|r| r.day).unique().collect::<Vec<NaiveDate>>();
	let prices: HashMap<NaiveDate, f64> = if app.no_price {
		HashMap::new()
	} else {
		api.fetch_prices(dates.as_slice())
			.context("Failed to fetch prices.")?
			.into_iter()
			.enumerate()
			.map(|(i, p)| (dates[i], p))
			.collect()
	};

	ensure!(!rewards.is_empty(), "No rewards found for specified account.");

	rewards
		.iter()
		.map(|r| {
			let price = prices.get(&r.day);
			Ok(SeparatedCsvRecord {
				date: r.day.format(&app.date_format).to_string(),
				time: r.time.format(&app.time_format).to_string(),
				block_number: format!("{}", r.block_num),
				amount: app.network.amount_to_network(&r.amount)?,
				price: price.copied().into(),
			})
		})
		.collect()
}

fn construct_progress_bar() -> ProgressBar {
	let bar = ProgressBar::new(1000);
	bar.set_style(
		ProgressStyle::default_bar()
			.template("{spinner:.blue} {msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})")
			.progress_chars("#>-"),
	);
	bar
}

// constructs a file name in the format: `dot-address-from_date-to_date-rewards.csv`
fn construct_file_name(app: &App, from: String, to: String) -> String {
	format!("{}->{}-{}-{}--{}-rewards", app.network.id(), app.currency, &app.address, from, to)
}
