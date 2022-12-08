//! Types related to market pairs.
use super::*;
use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

/// An asset/asset pairing.
#[derive(Serialize, Deserialize, Debug)]
pub struct Pair<'a> {
	id: u64,
	#[serde(borrow)]
	symbol: Cow<'a, str>,
	#[serde(borrow)]
	base: Asset<'a>,
	#[serde(borrow)]
	quote: Asset<'a>,
	#[serde(borrow)]
	route: Cow<'a, str>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::test_prelude::*;

	// Pairs -> List
	#[test]
	fn test_real_data_pair_deserialization() {
		let pairs = load_test_data(Call::Pairs(Pairs::List));
		let _: Response<Vec<Pair>> = assert_ok!(serde_json::from_slice(pairs.as_slice()));
	}
}
