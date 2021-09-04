# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Add moonriver, movr to network in `usage`

## [v0.3.0] - 2021-08-31
### Added
- `--no-price` flag. Skips fetching prices from CoinGecko.
- `--preview` flag. Outputs a table in CLI instead of a CSV format or writing to a file. Useful if you don't care about
  preserving the data you get and just want an overview of your rewards from the terminal.

## [v0.2.0] - 2021-08-31
### Added
- Add Moonriver Network. Available under `MOVR` or `moonriver` ids.
- Add option to change user-agent. `--user` or `-u` CLI option. Default user agent is `polkadot-rewards/version`

### Fixed
- Fix subscan URL. Used to be `network.subscan.io`, new url is `network.api.subscan.io`
- Fix hitting API Request limit. Change subscan requests to 3 per second.

## [v0.1.0] - 2021-08-08
### Added
- Polkadot Rewards CLI that scrapes rewards for Polkadot & Kusama from Subscan & Coingecko
- Support for 'sek', 'aud', 'ars', 'dot', 'bch', 'try', 'twd', 'vef', 'yfi', 'idr', 'sar', 'uah', 'inr', 'chf', 'krw', 'czk', 'bmd', 'bdt', 'cad', 'huf', 'ltc', 'pln', 'sgd', 'xlm', 'zar', 'link', 'ils', 'pkr', 'bnb', 'kwd', 'mmk', 'lkr', 'nzd', 'gbp', 'sats', 'mxn', 'cny', 'vnd', 'thb', 'xag', 'eur', 'ngn', 'nok', 'xrp', 'hkd', 'xau', 'clp', 'xdr', 'dkk', 'eth', 'jpy', 'eos', 'aed', 'bhd', 'php', 'brl', 'rub', 'btc', 'usd', 'bits', 'myr' curencies.
- supports RFC2822 date format
