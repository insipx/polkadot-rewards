//! Types and Utilities for Tests

pub mod test_prelude {
	pub use assert_ok::assert_ok;
	pub use std::{
		include_bytes,
		io::Read,
		path::{Path, PathBuf},
	};

	#[allow(unused)]
	pub enum Call {
		Assets(Assets),
		Exchanges(Exchanges),
		Markets(Markets),
		Pairs(Pairs),
	}

	#[allow(unused)]
	pub enum Assets {
		List,
		Details,
	}

	#[allow(unused)]
	pub enum Pairs {
		List,
		Details,
	}

	#[allow(unused)]
	pub enum Markets {
		List,
		Details,
		Price(OneOrAllMarkets),
		Trades,
		TwentyFourHourSummary(OneOrAllMarkets),
		OrderBook,
		OHLC,
	}

	#[allow(unused)]
	pub enum Exchanges {
		List,
		Details,
		Markets,
	}

	#[allow(unused)]
	pub enum OneOrAllMarkets {
		One,
		All,
	}

	trait CallExt {
		fn path(&self) -> PathBuf;
	}

	impl CallExt for Call {
		fn path(&self) -> PathBuf {
			match self {
				Call::Assets(a) => a.path(),
				Call::Exchanges(e) => e.path(),
				Call::Markets(m) => m.path(),
				Call::Pairs(p) => p.path(),
			}
		}
	}

	impl CallExt for Assets {
		fn path(&self) -> PathBuf {
			match self {
				Assets::List => Path::new("assets").join("list.json"),
				Assets::Details => Path::new("assets").join("details.json"),
			}
		}
	}

	impl CallExt for Pairs {
		fn path(&self) -> PathBuf {
			match self {
				Pairs::List => Path::new("pairs").join("list.json"),
				Pairs::Details => Path::new("pairs").join("details.json"),
			}
		}
	}

	impl CallExt for Markets {
		fn path(&self) -> PathBuf {
			match self {
				Markets::List => Path::new("markets").join("list.json"),
				Markets::Details => Path::new("markets").join("details.json"),
				Markets::Price(OneOrAllMarkets::One) => Path::new("markets").join("price.json"),
				Markets::Price(OneOrAllMarkets::All) => Path::new("markets").join("price-all.json"),
				Markets::Trades => Path::new("markets").join("trades.json"),
				Markets::TwentyFourHourSummary(OneOrAllMarkets::One) => Path::new("markets").join("24h-summary.json"),
				Markets::TwentyFourHourSummary(OneOrAllMarkets::All) =>
					Path::new("markets").join("24h-summary-all.json"),
				Markets::OrderBook => Path::new("markets").join("orderbook.json"),
				Markets::OHLC => Path::new("markets").join("ohlc.json"),
			}
		}
	}

	impl CallExt for Exchanges {
		fn path(&self) -> PathBuf {
			match self {
				Exchanges::List => Path::new("exchanges").join("list.json"),
				Exchanges::Details => Path::new("exchanges").join("details.json"),
				Exchanges::Markets => Path::new("exchanges").join("markets.json"),
			}
		}
	}

	pub fn load_test_data(call: Call) -> Vec<u8> {
		let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		path.push("data/");

		fn find_all_files(path: &Path) -> impl Iterator<Item = PathBuf> {
			let mut files = vec![];
			for entry in std::fs::read_dir(path).unwrap() {
				let entry = entry.unwrap();
				let path = entry.path();
				if path.is_dir() {
					files.extend(find_all_files(&path));
				} else {
					files.push(path);
				}
			}
			files.into_iter()
		}

		let mut files = find_all_files(path.as_path());
		let file = match call {
			Call::Assets(Assets::List) => files.find(|p| p.ends_with("assets/list.json")).unwrap(),
			Call::Assets(Assets::Details) => files.find(|p| p.ends_with("assets/details.json")).unwrap(),
			Call::Exchanges(Exchanges::List) => files.find(|p| p.ends_with("exchanges/list.json")).unwrap(),
			Call::Exchanges(Exchanges::Details) => files.find(|p| p.ends_with("exchanges/details.json")).unwrap(),
			Call::Exchanges(Exchanges::Markets) => files.find(|p| p.ends_with("exchanges/markets.json")).unwrap(),
			Call::Markets(Markets::List) => files.find(|p| p.ends_with("markets/list.json")).unwrap(),
			Call::Markets(Markets::Details) => files.find(|p| p.ends_with("markets/details.json")).unwrap(),
			Call::Markets(Markets::Price(OneOrAllMarkets::One)) =>
				files.find(|p| p.ends_with("markets/price.json")).unwrap(),
			Call::Markets(Markets::Price(OneOrAllMarkets::All)) =>
				files.find(|p| p.ends_with("markets/price-all.json")).unwrap(),
			Call::Markets(Markets::Trades) => files.find(|p| p.ends_with("markets/trades.json")).unwrap(),
			Call::Markets(Markets::TwentyFourHourSummary(OneOrAllMarkets::One)) =>
				files.find(|p| p.ends_with("markets/24h-summary.json")).unwrap(),
			Call::Markets(Markets::TwentyFourHourSummary(OneOrAllMarkets::All)) =>
				files.find(|p| p.ends_with("markets/24h-summary-all.json")).unwrap(),
			Call::Markets(Markets::OrderBook) => files.find(|p| p.ends_with("markets/orderbook.json")).unwrap(),
			Call::Markets(Markets::OHLC) => files.find(|p| p.ends_with("markets/ohlc.json")).unwrap(),
			Call::Pairs(Pairs::List) => files.find(|p| p.ends_with("pairs/list.json")).unwrap(),
			Call::Pairs(Pairs::Details) => files.find(|p| p.ends_with("pairs/details.json")).unwrap(),
		};
		std::fs::read(file).unwrap()
	}
}
