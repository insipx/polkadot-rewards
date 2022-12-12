//! Cryptowat.ch Client

use serde::{Deserialize, Serialize};
use std::{
	io::{Error as IoError, ErrorKind},
	path::Path,
};

mod assets;
mod exchanges;
mod markets;
mod pairs;

pub use assets::*;
pub use exchanges::*;
pub use markets::*;
pub use pairs::*;

// CryptowatchRS Version
const CRYPTOWATCH_RS_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientCredentials {
	/// The name of the API key
	pub key: String,
	/// The API key secret
	pub secret: String,
}

impl ClientCredentials {
	/// Load json-format kraken credentials file
	pub fn load_json_file<P: AsRef<Path>>(path: P) -> core::result::Result<Self, IoError> {
		let creds_file = std::fs::read_to_string(path)?;
		let creds_data: ClientCredentials = serde_json::from_str(&creds_file)?;
		if creds_data.key.is_empty() {
			return Err(IoError::new(ErrorKind::Other, "Missing credentials 'key' value"))
		}
		if creds_data.secret.is_empty() {
			return Err(IoError::new(ErrorKind::Other, "Missing credentials 'secret' value"))
		}
		Ok(creds_data)
	}
}
