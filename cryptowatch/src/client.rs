//! Cryptowat.ch Client

mod rest_client;

use crate::api;
use api::ApiError;
pub use rest_client::*;

use bytes::Bytes;
use http::{request, response::Parts, Response};
use once_cell::sync::Lazy;
use url::Url;

/// CryptowatchRS Version
const CRYPTOWATCH_RS_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

/// Cryptowat.ch URL
pub static CRYPTOWATCH_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://api.cryptowat.ch").unwrap());

/// CryptowatchClient that supports cryptowat.ch websockets and REST API functions
#[derive(Debug, Clone)]
pub struct CryptowatchClient {
	rest_client: CryptowatchRestClient,
}

impl CryptowatchClient {
	pub fn new_http(rest_client: CryptowatchRestClient) -> Self {
		Self { rest_client }
	}
}

impl api::Client for CryptowatchClient {
	type Error = super::Error;

	#[tracing::instrument]
	async fn rest(&self, mut request: request::Builder) -> Result<Response<Bytes>, ApiError> {
		self.rest_client.set_headers(request.headers_mut().unwrap());
		let response = self.rest_client.http.request(request.body(Default::default())?).await?;

		let (Parts { status, version, headers, .. }, body) = response.into_parts();

		let mut http_response = Response::builder().status(status).version(version);
		let http_headers = http_response.headers_mut().unwrap();
		http_headers.extend(headers.into_iter().map(|(name, value)| (name, value)));

		http_response
			.body(hyper::body::to_bytes(body).await?)
			.map_err(Into::<ApiError>::into)
	}
}

impl api::RestClient for CryptowatchClient {
	type Error = super::Error;

	fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError> {
		tracing::debug!(target: "cryptowatch::api", "REST api call {}", endpoint);
		CRYPTOWATCH_URL
			.join(endpoint)
			.map_err(|e| ApiError::invalid_url_endpoint(endpoint, e))
	}
}
