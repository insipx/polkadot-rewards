//! Cryptowat.ch Client

use hyper::Uri;
use once_cell::sync::Lazy;

mod assets;
mod exchanges;
mod markets;
mod pairs;
mod rest_client;

pub use assets::*;
pub use exchanges::*;
pub use markets::*;
pub use pairs::*;
pub use rest_client::*;

/// CryptowatchRS Version
const CRYPTOWATCH_RS_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

/// Cryptowat.ch URL
static CRYPTOWATCH_URI: Lazy<Uri> = Lazy::new(|| "https://api.cryptowat.ch".parse::<Uri>().unwrap());

pub struct Client;
