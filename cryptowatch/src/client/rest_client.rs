//! Cryptowat.ch lower-level REST Client
//! Slight abstraction over the REST API to make the API more ergonomic.

use super::*;
use crate::error::Error;
use http::HeaderMap;
use hyper::Uri;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
	io::{Error as IoError, ErrorKind},
	path::Path,
};

pub struct ClientConfig {}

/// Credentials to access API through a Cryptowat.ch Account
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ClientCredentials {
	/// The API key secret
	pub secret: String,
}

impl ClientCredentials {
	/// Load json-format kraken credentials file
	pub fn load_json_file<P: AsRef<Path>>(path: P) -> core::result::Result<Self, IoError> {
		let creds_file = std::fs::read_to_string(path)?;
		let creds_data: ClientCredentials = serde_json::from_str(&creds_file)?;
		if creds_data.secret.is_empty() {
			return Err(IoError::new(ErrorKind::Other, "Missing credentials 'secret' value"))
		}
		Ok(creds_data)
	}

	pub fn load_environment() -> core::result::Result<Self, IoError> {
		envy::prefixed("CRYPTOWATCH_")
			.from_env::<ClientCredentials>()
			.map_err(|e| IoError::new(ErrorKind::Other, format!("Missing credentials {:#?}", e)))
	}
}

/// Abstracts over the
pub struct RestClient {
	credentials: ClientCredentials,
	http: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
	headers: HeaderMap<http::HeaderValue>,
}

impl RestClient {
	/// Create a client and try to load the credentials from the environment.
	pub fn new() -> Result<Self, Error> {
		let credentials = ClientCredentials::load_environment()?;
		let https = hyper_tls::HttpsConnector::new();
		let http = hyper::Client::builder().build::<_, hyper::Body>(https);
		let headers = Self::build_headers(&credentials)?;
		Ok(Self { headers, credentials, http })
	}

	/// Create a client and try to load credentials from a JSON file.
	pub fn new_with_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
		let credentials = ClientCredentials::load_json_file(path)?;
		let https = hyper_tls::HttpsConnector::new();
		let http = hyper::Client::builder().build::<_, hyper::Body>(https);
		let headers = Self::build_headers(&credentials)?;
		Ok(Self { headers, credentials, http })
	}

	fn build_headers(credentials: &ClientCredentials) -> Result<HeaderMap, Error> {
		let mut headers = http::HeaderMap::new();
		headers.insert("USER_AGENT", format!("cryptowatchrs/{}", CRYPTOWATCH_RS_VERSION.unwrap_or("unknown")).parse()?);
		headers.insert("X-CW-API-KEY", credentials.secret.parse()?);
		Ok(headers)
	}
}
