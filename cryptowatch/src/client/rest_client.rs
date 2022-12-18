//! Cryptowat.ch lower-level REST Client
//! Slight abstraction over the REST API to make the API more ergonomic.

use super::*;
use crate::error::Error;
use http::{header::HeaderName, HeaderMap};
use serde::{Deserialize, Serialize};
use std::{
	io::{Error as IoError, ErrorKind},
	path::Path,
	time::Duration,
};

/// General configuration values.
#[derive(Debug, Clone)]
pub struct ClientConfig {
	/// The timeout to use for http connections.
	/// Defaults to 30 seconds.
	pub timeout: Duration,
	/// The API key from your cryptowat.ch account.
	/// create one here: https://cryptowat.ch/account/api-access
	pub credentials: ClientCredentials,
}

impl Default for ClientConfig {
	fn default() -> Self {
		ClientConfig { timeout: Duration::from_secs(30), credentials: ClientCredentials::default() }
	}
}

/// Credentials to access API through a Cryptowat.ch Account
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct ClientCredentials {
	/// The API key secret
	pub secret: Option<String>,
}

impl ClientCredentials {
	/// Create new Client Credentials
	pub fn new<S: Into<String>>(secret: S) -> Self {
		Self { secret: Some(secret.into()) }
	}

	/// Tries to load credentials from a JSON file. It is assumed you want to load credentials if you use this function,
	/// so if the secret does not exist an error is returned.
	/// # Errors
	/// - If the file does not exist
	/// - If the file is not valid JSON
	/// - If the file does not contain a valid `ClientCredentials` struct
	/// - If the secret is missing
	pub fn load_json_file<P: AsRef<Path>>(path: P) -> Result<Self, IoError> {
		let creds_file = std::fs::read_to_string(path)?;
		let creds_data: ClientCredentials = serde_json::from_str(&creds_file)?;
		if creds_data.secret.is_none() {
			return Err(IoError::new(ErrorKind::Other, "Missing credentials 'secret' value"))
		}
		Ok(creds_data)
	}

	/// Tries to load credentials from the environment. It is assumed you want to load credentials if you use this
	/// function, so if the secret does not exist an error is returned.
	/// # Errors
	/// - If the environment variable `CRYPTOWATCH_SECRET` is not set
	pub fn load_environment() -> Result<Self, IoError> {
		envy::prefixed("CRYPTOWATCH_")
			.from_env::<ClientCredentials>()
			.map_err(|e| IoError::new(ErrorKind::Other, format!("Missing credentials {e:#?}")))
	}
}

/// Abstracts over the REST API for ergonomics.
#[derive(Debug, Clone)]
pub struct CryptowatchRestClient {
	pub(crate) http: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
	headers: HeaderMap<http::HeaderValue>,
}

impl CryptowatchRestClient {
	/// Create a client and try to load the credentials from the environment.
	pub fn new(credentials: ClientCredentials) -> Result<Self, Error> {
		let config = ClientConfig::default();
		let https = hyper_tls::HttpsConnector::new();
		let http = hyper::Client::builder()
			.pool_idle_timeout(config.timeout)
			.build::<_, hyper::Body>(https);
		let headers = Self::build_headers(&credentials)?;
		Ok(Self { headers, http })
	}

	/// Create a client and try to load credentials from a JSON file.
	pub fn with_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
		let config = ClientConfig::default();
		let credentials = ClientCredentials::load_json_file(path)?;
		let https = hyper_tls::HttpsConnector::new();
		let http = hyper::Client::builder()
			.pool_idle_timeout(config.timeout)
			.build::<_, hyper::Body>(https);
		let headers = Self::build_headers(&credentials)?;
		Ok(Self { headers, http })
	}

	/// Create a REST client taking the secret from the environment.
	pub fn with_environment() -> Result<Self, Error> {
		let config = ClientConfig::default();
		let credentials = ClientCredentials::load_environment()?;
		let https = hyper_tls::HttpsConnector::new();
		let http = hyper::Client::builder()
			.pool_idle_timeout(config.timeout)
			.build::<_, hyper::Body>(https);
		let headers = Self::build_headers(&credentials)?;
		Ok(Self { headers, http })
	}

	/// Create a REST client not associated with any cryptowat.ch account.
	/// This uses the public allowance allowed per-user.
	pub fn with_public() -> Result<Self, Error> {
		let config = ClientConfig::default();
		let credentials = ClientCredentials::default();
		let https = hyper_tls::HttpsConnector::new();
		let http = hyper::Client::builder()
			.pool_idle_timeout(config.timeout)
			.build::<_, hyper::Body>(https);
		let headers = Self::build_headers(&credentials)?;
		Ok(Self { headers, http })
	}

	fn build_headers(credentials: &ClientCredentials) -> Result<HeaderMap, Error> {
		let mut headers = http::HeaderMap::new();
		headers.insert(
			HeaderName::try_from("USER_AGENT").unwrap(),
			format!("cryptowatchrs/{}", CRYPTOWATCH_RS_VERSION.unwrap_or("unknown")).parse()?,
		);
		if let Some(secret) = &credentials.secret {
			headers.insert(HeaderName::try_from("X-CW-API-Key").unwrap(), secret.parse()?);
		}
		Ok(headers)
	}

	pub(crate) fn set_headers(&self, headers: &mut HeaderMap<http::HeaderValue>) {
		headers.extend(self.headers.iter().map(|(name, value)| (name.clone(), value.clone())));
	}
}
