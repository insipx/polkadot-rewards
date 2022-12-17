use std::str::FromStr;

use crate::types::*;

impl FromStr for Pair {
	type Err = ();
	fn from_str(pair: &str) -> Result<Self, Self::Err> {
		Ok(Pair { pair: pair.to_string() })
	}
}

impl From<String> for Pair {
	fn from(pair: String) -> Self {
		Pair { pair }
	}
}

impl From<&str> for Pair {
	fn from(pair: &str) -> Self {
		Pair { pair: pair.to_string() }
	}
}
