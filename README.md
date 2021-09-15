<div align="center">

# Polkadot Rewards
### Crawl rewards for a Polkadot/Kusama/Moonriver accounts from subscan. Outputs to a CSV.

</div>

<div align="center">

[![Rust](https://github.com/insipx/polkadot-rewards/actions/workflows/rust.yml/badge.svg)](https://github.com/insipx/polkadot-rewards/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/polkadot-rewards)](https://crates.io/crates/polkadot-rewards)
[![Crates.io](https://img.shields.io/crates/d/polkadot-rewards)](https://crates.io/crates/polkadot-rewards)
[![Crates.io](https://img.shields.io/crates/l/polkadot-rewards)](https://crates.io/crates/polkadot-rewards)

</div>


# Installation

## Binaries
Pre-Built binaries for Linux, MacOS, and Windows for the latest release are available on the github page [here](https://github.com/insipx/polkadot-rewards/releases).

## Cargo
`cargo install polkadot-rewards` will place the binary in your `$PATH` for cargo binaries.

## Compiling
We'll need to first install Rust. You may need to add Cargo's bin directory to your PATH environment variable. Restarting your computer will do this for you automatically.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once done, clone this repo and build the polkadot-rewards software

```bash
# clone this repo
git clone https://github.com/insipx/polkadot-rewards.git

#  move to the newly created directory
cd polkadot-rewards

# build the polkadot-rewards binary
cargo build --release

# move to the target folder where you will find the binary
cd ./target/release
```

You can now use the binary `./polkadot-rewards` from here.

# Usage

tip: the date format can be directly copied from https://subscan.io by hovering over the `date` section of the reward.

./polkadot-rewards --from "YYYY-MM-DD HH:MM:SS" --network ksm --address "rewards-address" --currency eur

```
Usage: polkadot-rewards [-f <from>] [-t <to>] [-n <network>] -c <currency> -a <address> [-u <user>] [--date-format <date-format>] [--time-format <time-format>] [-p <folder>] [-s] [--no-price] [--no-group] [--preview] [-v]

Polkadot Staking Rewards CLI-App

Options:
  -f, --from        date to start crawling for staking rewards. Format:
                    "YYY-MM-DD HH:MM:SS"
  -t, --to          date to stop crawling for staking rewards. Format:
                    "YYY-MM-DD HH:MM:SS"
  -n, --network     network to crawl for rewards. One of: [Polkadot, Kusama,
                    Moonriver, MOVR, KSM, DOT]
  -c, --currency    the fiat currency which should be used for prices
  -a, --address     network-formatted address to get staking rewards for.
  -u, --user        change the user agent for HTTPS requests
  --date-format     date format to use in output CSV data. Uses rfc2822 by
                    default.  EX: "%Y-%m-%d %H:%M:%S".
  --time-format     time format to use with `--no-group` flag. Default
                    "%H:%M:%S".
  -p, --folder      directory to output completed CSV to.
  -s, --stdout      output the CSV file to STDOUT. Disables creating a new file.
  --no-price        do not gather price data
  --no-group        do not group blocks by day. Give each block its own column.
                    Adds additional exact UTC `time` column.
  --preview         preview the rewards in your terminal instead of outputting
                    CSV format.
  -v, --verbose     get extra information about the program execution.
  --help            display usage information
```
