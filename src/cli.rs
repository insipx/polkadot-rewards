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
	primitives::{CsvRecord, RewardEntry},
};
use anyhow::{anyhow, bail, ensure, Context, Error};
use argh::FromArgs;
use chrono::naive::NaiveDateTime;
use env_logger::{Builder, Env};
use indicatif::{ProgressBar, ProgressStyle};
use sp_arithmetic::{FixedPointNumber, FixedU128};
use std::{fs::File, io, path::PathBuf, str::FromStr};

const OUTPUT_DATE: &str = "%Y-%m-%d";

#[derive(FromArgs, PartialEq, Debug)]
/// Polkadot Staking Rewards CLI-App
pub struct App {
	#[argh(option, from_str_fn(date_from_string), short = 'f')]
	/// date to start crawling for staking rewards. Format: "YYY-MM-DD HH:MM:SS"
	pub from: Option<NaiveDateTime>,
	/// date to stop crawling for staking rewards. Format: "YYY-MM-DD HH:MM:SS"
	#[argh(option, from_str_fn(date_from_string), short = 't')]
	pub to: Option<NaiveDateTime>,
	/// network to crawl for rewards. One of: [Polkadot, Kusama, KSM, DOT]
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
	date_format: String,
	/// directory to output completed CSV to.
	#[argh(option, default = "default_file_location()", short = 'p')]
	folder: PathBuf,
	/// output the CSV file to STDOUT. Disables creating a new file.
	#[argh(switch, short = 's')]
	stdout: bool,
	#[argh(switch)]
	/// don't gather price data
	no_price: bool,
	/// get extra information about the program's execution.
	#[argh(switch, short = 'v')]
	verbose: bool,
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
	/// The Moonbeam Network
	Moonriver,
}

impl Network {
	pub fn id(&self) -> &'static str {
		match self {
			Self::Polkadot => "polkadot",
			Self::Kusama => "kusama",
			Self::Moonriver => "moonriver",
		}
	}

	fn amount_to_network(&self, amount: &u128) -> Result<f64, Error> {
		let denominator = match self {
			Self::Polkadot => 10_000_000_000u128,  // 1 Billion DOT
			Self::Kusama => 1_000_000_000_000u128, // 10 Mil KSM
			Self::Moonriver => 1_000_000_000_000_000_000u128,
		};
		let frac = FixedU128::checked_from_rational(*amount, denominator)
			.ok_or_else(|| anyhow!("Amount '{}' overflowed FixedU128", amount))?
			.to_fraction();
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
			_ => bail!(
				"Network must be one of: 'kusama', 'polkadot', 'moonriver', 'karura', 'khala', 'shiden' or their
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

	let rewards = api.fetch_all_rewards().context("Failed to fetch rewards.")?;
	let prices = if !app.no_price {
		api.fetch_prices(&rewards).context("Failed to fetch prices.")?.into_iter().map(Some).collect::<Vec<_>>()
	} else {
		[0..rewards.len()].iter().map(|_| None).collect::<Vec<Option<_>>>()
	};

	ensure!(!rewards.is_empty(), "No rewards found for specified account.");

	let file_name = construct_file_name(&app, &rewards);
	app.folder.push(&file_name);
	app.folder.set_extension("csv");

	let mut wtr = Output::new(&app).context("Failed to create output.")?;

	rewards
		.into_iter()
		.zip(prices)
		.map(|(reward, price)| {
			Ok(CsvRecord {
				block_nums: reward.block_nums.into_iter().fold(String::new(), |acc, i| format!("{}+{}", acc, i))[1..]
					.to_string(),
				date: reward.day.format(&app.date_format).to_string(),
				amount: app.network.amount_to_network(&reward.amount)?,
				price,
			})
		})
		.try_for_each(|r: Result<_, Error>| wtr.serialize(r?).context("Failed to format CsvRecord"))?;

	if app.stdout {
		progress.map(|p| p.finish_with_message("Writing data to STDOUT"));
	} else {
		progress.map(move |p| p.finish_with_message(format!("Wrote data to file: {}", file_name)));
	}
	Ok(())
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
fn construct_file_name(app: &App, rewards: &[RewardEntry]) -> String {
	format!(
		"{}->{}-{}-{}--{}-rewards",
		app.network.id(),
		app.currency,
		&app.address,
		rewards.first().unwrap().day.format(OUTPUT_DATE),
		rewards.last().unwrap().day.format(OUTPUT_DATE)
	)
}

enum Output {
	FileOut(csv::Writer<File>),
	StdOut(csv::Writer<std::io::Stdout>),
}

impl Output {
	fn new(app: &App) -> Result<Self, Error> {
		let mut builder = csv::WriterBuilder::new();
		builder.delimiter(b';');
		if app.stdout {
			Ok(Output::StdOut(builder.from_writer(io::stdout())))
		} else {
			let file = File::create(&app.folder)?;
			Ok(Output::FileOut(builder.from_writer(file)))
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
