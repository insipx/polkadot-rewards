//! Cryptowat.ch lower-level REST Client
//! Slight abstraction over the REST API to make the API more ergonomic.

use super::*;
use crate::error::Error;
use http::HeaderMap;
use hyper::Uri;
use serde::{Deserialize, Serialize};
use std::{
	io::{Error as IoError, ErrorKind},
	path::Path,
	time::Duration,
};

/// General configuration values.
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
	/// Tries to load credentials from a JSON file. It is assumed you want to load credentials if you use this function,
	/// so if the secret does not exist an error is returned.
	/// # Errors
	/// - If the file does not exist
	/// - If the file is not valid JSON
	/// - If the file does not contain a valid `ClientCredentials` struct
	/// - If the secret is missing
	pub fn load_json_file<P: AsRef<Path>>(path: P) -> core::result::Result<Self, IoError> {
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
	pub fn load_environment() -> core::result::Result<Self, IoError> {
		envy::prefixed("CRYPTOWATCH_")
			.from_env::<ClientCredentials>()
			.map_err(|e| IoError::new(ErrorKind::Other, format!("Missing credentials {:#?}", e)))
	}
}

/// Get the endpoint and data for some cryptowatch data type.
pub trait CryptowatchDataSet {
	fn endpoint() -> Uri;
	async fn get() -> Result<Self, Error> where Self: Sized;
	// possibly use borrowed data
	// async fn get() -> Result<hyper::Response<hyper::Body>, Error>;
}

pub trait UriExt {
	fn with_query(&self, query: &str) -> Result<Self, Error>
	where
		Self: Sized;
}

pub trait ResponseExt {
	type Output;
	fn unpack(self) -> Self::Output;
}

impl UriExt for hyper::Uri {
	fn with_query(&self, query: &str) -> Result<Self, Error> {
		let uri = self.clone().into_parts();
		let uri = http::uri::Builder::new()
			.scheme(uri.scheme.ok_or(Error::NoScheme)?)
			.authority(uri.authority.ok_or(Error::NoAuthority)?)
			.path_and_query(query)
			.build()?;
		Ok(uri)
	}
}

/// Abstracts over the REST API for ergonomics.
pub struct RestClient {
	credentials: ClientCredentials,
	pub http: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
	pub headers: HeaderMap<http::HeaderValue>,
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
		if let Some(secret) = &credentials.secret {
			headers.insert("X-CW-API-Key", secret.parse()?);
		}
		Ok(headers)
	}

	pub fn set_credentials(&mut self, credentials: ClientCredentials) -> Result<(), Error> {
		self.credentials = credentials;
		self.headers = Self::build_headers(&self.credentials)?;
		Ok(())
	}

	pub async fn request<T: CryptowatchDataSet>(&self) -> Result<T, Error> {
		let data = T::get().await?;
		Ok(data)
	}
}
