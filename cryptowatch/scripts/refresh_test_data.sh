#! /bin/bash

# Refresh static test data from CryptoWatch REST API

# Assets
curl -G https://api.cryptowat.ch/assets | jq > data/assets/list.json
curl -G https://api.cryptowat.ch/assets/btc | jq > data/assets/details.json

# Pairs
curl -G https://api.cryptowat.ch/pairs | jq > data/pairs/list.json
curl -G https://api.cryptowat.ch/pairs/btceur | jq > data/pairs/details.json

# Markets
curl -G https://api.cryptowat.ch/markets | jq > data/markets/list.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur | jq > data/markets/details.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur/price | jq > data/markets/price.json
curl -G https://api.cryptowat.ch/markets/prices | jq > data/markets/price-all.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur/trades | jq > data/markets/trades.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur/summary | jq > data/markets/24h-summary.json
curl -G https://api.cryptowat.ch/markets/summaries | jq > data/markets/24h-summary-all.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur/orderbook | jq > data/markets/orderbook.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur/orderbook/liquidity | jq > data/markets/orderbook-liquidity.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur/orderbook/calculator -d "amount=1000" | jq > data/markets/orderbook-calculator.json
curl -G https://api.cryptowat.ch/markets/kraken/btceur/ohlc | jq > data/markets/ohlc.json


# Exchanges
curl -G https://api.cryptowat.ch/exchanges | jq > data/exchanges/list.json
curl -G https://api.cryptowat.ch/exchanges/kraken | jq > data/exchanges/details.json
curl -G https://api.cryptowat.ch/markets/kraken | jq > data/exchanges/markets.json

