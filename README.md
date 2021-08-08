<div align="center">

# Polkadot Rewards 
### Crawl rewards for a Polkadot/Kusama account from subscan. Outputs to a CSV.

</div>


[![Rust](https://github.com/insipx/polkadot-rewards/actions/workflows/rust.yml/badge.svg)](https://github.com/insipx/polkadot-rewards/actions/workflows/rust.yml)

# Installation

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
cargo build

# move to the target folder where you will find the binary
cd ./target/debug
```

You can now use the binary `./polkadot-rewards` from here.

# Usage

./polkadot-rewards --from "YYYY-MM-DD HH:MM:SS" --network "KSM" --address "Addr-Without-Leading-0x" --currency "eur"

```bash
Usage: polkadot-rewards [-f <from>] [-t <to>] [-n <network>] -c <currency> -a <address> [--date-format <date-format>] [-p <folder>] [-s] [-v]

Polkadot Staking Rewards CLI-App

Options:
  -f, --from        date to start crawling for staking rewards. Format: "YYY-MM-DD HH:MM:SS"
  -t, --to          date to stop crawling for staking rewards. Format: "YYY-MM-DD HH:MM:SS"
  -n, --network     network to crawl for rewards. One of: [Polkadot, Kusama, KSM, DOT]
  -c, --currency    the fiat currency which should be used for prices
  -a, --address     network-formatted address to get staking rewards for.
  --date-format     date format to use in output CSV data. Uses rfc2822 by default.  EX: "%Y-%m-%d %H:%M:%S".
  -p, --folder      directory to output completed CSV to.
  -s, --stdout      output the CSV file to STDOUT. Disables creating a new file.
  -v, --verbose     get extra information about the program's execution.
  --help            display usage information
```
