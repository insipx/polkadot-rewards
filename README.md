<div align="center">

# Polkadot Rewards 
### Crawl rewards for a Polkadot/Kusama account from subscan. Outputs to a CSV.

</div>


[![Rust](https://github.com/insipx/polkadot-rewards/actions/workflows/rust.yml/badge.svg)](https://github.com/insipx/polkadot-rewards/actions/workflows/rust.yml)


# Usage

./polkadot-rewards --from "YYYY-MM-DD HH:MM:SS" --network "KSM" --address "Addr-Without-Leading-0x"

```bash
Usage: polkadot-rewards -f <from> [-t <to>] [-n <network>] -a <address> [--date-format <date-format>] [-p <folder>] [-s] [-v]

Polkadot Staking Rewards CLI-App

Options:
  -f, --from        date to start crawling for staking rewards. Format: "YYYY-MM-DD HH:MM:SS"
  -t, --to          date to stop crawling for staking rewards. Defaults
                    to current time. Format: "YYYY-MM-DD HH:MM:SS"
  -n, --network     the network for rewards. One of: {Polkadot, Kusama, KSM, DOT }
  -a, --address     network-formatted address to get staking rewards for.
  --date-format     date format to use in output CSV data. Uses rfc2822 by
                    default.  Example: "%Y-%m-%d %H:%M:%S".
  -p, --folder      directory to output completed CSV to.
  -s, --stdout      output the CSV file to STDOUT. Disables creating a new file.
  -v, --verbose     get extra information about the program's execution
  --help            display usage information
```

