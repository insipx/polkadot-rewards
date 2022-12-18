//! Idiomatic Rust description of `cryptowat.ch` REST API.
//! This is based upon the `gitlab` rust API.
//! Described here in this [article](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)

pub mod error;
pub mod params;

mod assets;
mod combinators;
mod exchanges;
mod markets;
mod pairs;
mod traits;

use crate::types;
use http::{Request, Uri};
use std::fmt;
use url::Url;

// re-exports
pub use error::ApiError;
pub use params::{PathParams, QueryParams};
use serde::de::DeserializeOwned;
pub use traits::{Client, Endpoint, Query, RestClient};

/// Import all endpoints
pub mod prelude {
	use super::*;
	pub use assets::*;
	pub use exchanges::*;
	pub use markets::*;
	pub use pairs::*;
}

impl<E, T, C> Query<T, C> for E
where
	E: Endpoint + fmt::Debug,
	T: DeserializeOwned + fmt::Debug,
	C: Client + RestClient + fmt::Debug,
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
