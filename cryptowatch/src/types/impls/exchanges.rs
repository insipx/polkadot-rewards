//! Implementations for `Exchange` types.
use crate::{error::Error, types::*};

impl TryFrom<String> for Exchange {
	type Error = Error;
	fn try_from(s: String) -> Result<Self, Self::Error> {
		match s.as_str() {
			"bitflyer" => Ok(Exchange::BitFlyer),
			"bittrex" => Ok(Exchange::Bittrex),
			"gemini" => Ok(Exchange::Gemini),
			"luno" => Ok(Exchange::Luno),
			"gateio" => Ok(Exchange::Gateio),
			"bitfinex" => Ok(Exchange::Bitfinex),
			"kraken" => Ok(Exchange::Kraken),
			"cexio" => Ok(Exchange::Cexio),
			"bisq" => Ok(Exchange::Bisq),
			"bitmex" => Ok(Exchange::BitMEX),
			"okex" => Ok(Exchange::Okex),
			"kraken-futures" => Ok(Exchange::KrakenFutures),
			"liquid" => Ok(Exchange::Liquid),
			"quoine" => Ok(Exchange::Quoine),
			"bitbay" => Ok(Exchange::BitBay),
			"hitbtc" => Ok(Exchange::HitBTC),
			"binance" => Ok(Exchange::Binance),
			"binance-us" => Ok(Exchange::BinanceUS),
			"huobi" => Ok(Exchange::Huobi),
			"poloniex" => Ok(Exchange::Poloniex),
			"coinbase-pro" => Ok(Exchange::CoinbasePro),
			"bitstamp" => Ok(Exchange::Bitstamp),
			"bitz" => Ok(Exchange::BitZ),
			"bithumb" => Ok(Exchange::Bithumb),
			"coinone" => Ok(Exchange::Coinone),
			"dex-aggregated" => Ok(Exchange::DexAggregated),
			"okcoin" => Ok(Exchange::OkCoin),
			"ftx" => Ok(Exchange::Ftx),
			"uniswap-v2" => Ok(Exchange::UniswapV2),
			"bybit" => Ok(Exchange::Bybit),
			"crypto-com" => Ok(Exchange::CryptoCom),
			"deribit" => Ok(Exchange::Deribit),
			"kucoin" => Ok(Exchange::KuCoin),
			"okx" => Ok(Exchange::Okex),
			"zonda" => Ok(Exchange::Zonda),
			_ => Err(Error::UnsupportedExchange(s)),
		}
	}
}

impl<'a> TryFrom<&'a str> for Exchange {
	type Error = Error;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		match s {
			"bitflyer" => Ok(Exchange::BitFlyer),
			"bittrex" => Ok(Exchange::Bittrex),
			"gemini" => Ok(Exchange::Gemini),
			"luno" => Ok(Exchange::Luno),
			"gateio" => Ok(Exchange::Gateio),
			"bitfinex" => Ok(Exchange::Bitfinex),
			"kraken" => Ok(Exchange::Kraken),
			"cexio" => Ok(Exchange::Cexio),
			"bisq" => Ok(Exchange::Bisq),
			"bitmex" => Ok(Exchange::BitMEX),
			"okex" => Ok(Exchange::Okex),
			"kraken-futures" => Ok(Exchange::KrakenFutures),
			"liquid" => Ok(Exchange::Liquid),
			"quoine" => Ok(Exchange::Quoine),
			"bitbay" => Ok(Exchange::BitBay),
			"hitbtc" => Ok(Exchange::HitBTC),
			"binance" => Ok(Exchange::Binance),
			"binance-us" => Ok(Exchange::BinanceUS),
			"huobi" => Ok(Exchange::Huobi),
			"poloniex" => Ok(Exchange::Poloniex),
			"coinbase-pro" => Ok(Exchange::CoinbasePro),
			"bitstamp" => Ok(Exchange::Bitstamp),
			"bitz" => Ok(Exchange::BitZ),
			"bithumb" => Ok(Exchange::Bithumb),
			"coinone" => Ok(Exchange::Coinone),
			"dex-aggregated" => Ok(Exchange::DexAggregated),
			"okcoin" => Ok(Exchange::OkCoin),
			"ftx" => Ok(Exchange::Ftx),
			"uniswap-v2" => Ok(Exchange::UniswapV2),
			"bybit" => Ok(Exchange::Bybit),
			"crypto-com" => Ok(Exchange::CryptoCom),
			"deribit" => Ok(Exchange::Deribit),
			"kucoin" => Ok(Exchange::KuCoin),
			"okx" => Ok(Exchange::Okex),
			"zonda" => Ok(Exchange::Zonda),
			_ => Err(Error::UnsupportedExchange(s.to_string())),
		}
	}
}
