//! Idiomatic Rust description of `cryptowat.ch` REST API.
//! This is based upon the `gitlab` rust API.
//! Described here in this [article](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)

pub mod error;
pub mod params;

pub use error::ApiError;
pub use params::{PathParams, QueryParams};

mod assets;
mod exchanges;
mod markets;
mod pairs;

use crate::types;
use bytes::Bytes;
use http::{request, response::Response, Request, Uri};
use serde::de::DeserializeOwned;
use std::{borrow::Cow, error::Error as StdError, fmt};
use url::Url;

/// Import all endpoints
pub mod prelude {
	use super::*;
	pub use assets::*;
	pub use exchanges::*;
	pub use markets::*;
	pub use pairs::*;
}

/// An endpoint of the `cryptowat.ch` API.
/// All cryptowat.ch API Endpoints are "GET"
// Slimmed down version of Gitlab rust crates `Endpoint` trait
pub trait Endpoint {
	// we only use GET in cryptowat.ch
	// fn method(&self) -> Method;
	fn endpoint(&self) -> Cow<'static, str>;

	/// Any possible query parameters for the endpoint
	fn parameters(&self) -> QueryParams {
		QueryParams::default() // Many endpoints don't have parameters
	}

	// TODO: Maybe needed in the future, but doubt it
	// fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
	// 	Ok(None) // Many endpoints also do not have request bodies
	// }

	/// Any additional possible paths for this endpoint.
	fn path(&self) -> PathParams {
		PathParams::default() // Many endpoints do not have an additional path
	}
}

pub trait Client {
	type Error: StdError + Send + Sync + 'static;
	/// Get the REST Endpoint this client interacts with
	fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError>;
	/// Execute the REST API request
	async fn rest(&self, request: request::Builder) -> Result<Response<Bytes>, ApiError>;
}

pub trait Query<T, C> {
	async fn query(&self, client: &C) -> Result<T, ApiError>;
}

impl<E, T, C> Query<T, C> for E
where
	E: Endpoint + fmt::Debug,
	T: DeserializeOwned + fmt::Debug,
	C: Client + fmt::Debug,
{
	#[tracing::instrument]
	async fn query(&self, client: &C) -> Result<T, ApiError> {
		let mut url = client.rest_endpoint(&self.endpoint())?;
		self.path().add_to_url(&mut url)?;
		self.parameters().add_to_url(&mut url);
		tracing::debug!(target: "cryptowatch::api", "Query URL Is: {}", url);
		let request = Request::builder().method("GET").uri(url_to_http_uri(url));
		let response = client.rest(request).await?;
		let status = response.status();

		let value = if let Ok(value) = serde_json::from_slice(response.body()) {
			value
		} else {
			return Err(ApiError::server_error(status, response.body()))
		};

		if !status.is_success() {
			return Err(ApiError::HttpError(status))
		}
		let response: types::Response<T> = serde_json::from_value(value).map_err(ApiError::data_type::<T>)?;
		Ok(response.unpack())
	}
}

fn url_to_http_uri(url: Url) -> Uri {
	url.as_str().parse::<Uri>().expect("failed to parse a url::Url as an http::Uri")
}
