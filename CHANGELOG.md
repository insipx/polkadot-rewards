# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.5.4] - 2023-04-24

### Fixed
- ([#179](https://github.com/insipx/polkadot-rewards/pull/179)) Increase pricing pause to 5s from 2s.

### Changed
- update Cargo dependencies

## [v0.5.3] - 2022-12-06

### Added
- Calamari network ([#141](https://github.com/insipx/polkadot-rewards/pull/141))


## [v0.5.2] - 2022-09-03

### Changed

- Update dependencies to latest

### Added

- local kv cache for price data ([#122](https://github.com/insipx/polkadot-rewards/pull/122))
- Add aleph-zero ([#106](https://github.com/insipx/polkadot-rewards/pull/106))

## [v0.5.1] - 2022-06-28

### Added

- add API key option https://github.com/insipx/polkadot-rewards/pull/102/files

## [v0.5.0] - 2022-02-22

### Changed

- Updated dependencies

### Added

- Added `Moonbeam` network

## [v0.4.0] - 2021-09-15

### Changed

- Add moonriver, movr to network in `usage`
- rearrange columns for readability. `amount` and `price` are now next to each
  other.

### Added

- `--no-group` flag. `--no-group` does not group block-numbers by day. Instead,
  each block number gets its own row. This additionally adds a `time` column,
  with the Hour, Minute, Second timestamp of the block.
- `--time-format` option. This allows the configuration of the time format when
  using `--no-group` flag

## [v0.3.0] - 2021-08-31

### Added

- `--no-price` flag. Skips fetching prices from CoinGecko.
- `--preview` flag. Outputs a table in CLI instead of a CSV format or writing to
  a file. Useful if you don't care about preserving the data you get and just
  want an overview of your rewards from the terminal.

## [v0.2.0] - 2021-08-31

### Added

- Add Moonriver Network. Available under `MOVR` or `moonriver` ids.
- Add option to change user-agent. `--user` or `-u` CLI option. Default user
  agent is `polkadot-rewards/version`

### Fixed

- Fix subscan URL. Used to be `network.subscan.io`, new url is
  `network.api.subscan.io`
- Fix hitting API Request limit. Change subscan requests to 3 per second.

## [v0.1.0] - 2021-08

### Added

- Polkadot Rewards CLI that scrapes rewards for Polkadot & Kusama from Subscan &
  Coingecko
- Support for 'sek', 'aud', 'ars', 'dot', 'bch', 'try', 'twd', 'vef', 'yfi',
  'idr', 'sar', 'uah', 'inr', 'chf', 'krw', 'czk', 'bmd', 'bdt', 'cad', 'huf',
  'ltc', 'pln', 'sgd', 'xlm', 'zar', 'link', 'ils', 'pkr', 'bnb', 'kwd', 'mmk',
  'lkr', 'nzd', 'gbp', 'sats', 'mxn', 'cny', 'vnd', 'thb', 'xag', 'eur', 'ngn',
  'nok', 'xrp', 'hkd', 'xau', 'clp', 'xdr', 'dkk', 'eth', 'jpy', 'eos', 'aed',
  'bhd', 'php', 'brl', 'rub', 'btc', 'usd', 'bits', 'myr' curencies.
- supports RFC2822 date format
