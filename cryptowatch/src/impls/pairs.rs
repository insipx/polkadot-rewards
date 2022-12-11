use std::{str::FromStr, borrow::Cow};

use crate::types::*;


impl FromStr for Pair<'_> {
	type Err = ();
	fn from_str(pair: &str) -> Result<Self, Self::Err> {
		Ok(Pair { pair: Cow::Owned(pair.to_string()) })
	}
}

impl From<String> for Pair<'_> {
	fn from(pair: String) -> Self {
		Pair { pair: Cow::Owned(pair) }
	}
}

impl<'a> From<&'a str> for Pair<'a> {
	fn from(pair: &'a str) -> Self {
		Pair { pair: Cow::Borrowed(pair) }
	}
}

impl Pair<'_> {
	pub fn into_owned(self) -> Pair<'static> {
		Pair { pair: Cow::Owned(self.pair.into_owned()) }
	}
}
