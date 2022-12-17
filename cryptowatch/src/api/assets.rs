//! "/assets" endpoint and "/assets/:asset" endpoint. Combined because assets endpoint is small. Lists all assests or
//! lists details about a single asset.

use crate::api::{Endpoint, PathParams, QueryParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option), build_fn(validate = "Self::validate"))]
pub struct Assets<'a> {
	/// If the request is paginated, the previously received cursor value.
	#[builder(setter(into), default, private)]
	cursor: Option<Cow<'a, str>>,
	/// Maximum number of results to be returned (Default: 5000, Maximum: 5000)
	#[builder(default)]
	limit: Option<u64>,
	/// Details about a specific asset.
	/// If this is set, all other parameters are ignored.
	#[builder(setter(into), default)]
	asset: Option<Cow<'a, str>>,
}

impl<'a> Assets<'a> {
	/// Create a builder for Assets
	pub fn builder() -> AssetsBuilder<'a> {
		AssetsBuilder::default()
	}
}

impl<'a> Endpoint for Assets<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		"assets".into()
	}

	fn parameters(&self) -> QueryParams {
		let mut params = QueryParams::default();

		params.push_opt("limit", self.limit).push_opt("cursor", self.cursor.as_ref());
		params
	}

	fn path(&self) -> PathParams {
		let mut paths = PathParams::default();
		paths.push_opt(self.asset.as_ref());
		paths
	}
}

impl<'a> AssetsBuilder<'a> {
	fn validate(&self) -> Result<(), String> {
		if let Some(_) = self.asset {
			if !(self.limit.is_none() && self.cursor.is_none()) {
				tracing::warn!(target: "cryptowatch::api", "Ignoring limit and cursor parameters because asset is set");
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		prelude::{Asset, AssetDetails, CryptowatchClient, Query, RestClient},
		tests::prelude::*,
	};

	#[test]
	fn can_build() {
		Assets::builder().asset("btc").build().unwrap();
		Assets::builder().limit(100).cursor("test").build().unwrap();
		Assets::builder().build().unwrap();
	}

	#[test]
	fn endpoint_list() {
		init();
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = Assets::builder().build().unwrap();
		let assets: Vec<Asset> = tokio_test::block_on(endpoint.query(&client)).unwrap();
		assert!(!assets.is_empty());
	}

	#[test]
	fn endpoint_details() {
		init();
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = Assets::builder().asset("btc").build().unwrap();
		let _: AssetDetails = tokio_test::block_on(endpoint.query(&client)).unwrap();
	}

	#[test]
	fn endpoint_details_still_works_with_other_params() {
		init();
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = Assets::builder().asset("btc").limit(0).cursor("hi").build().unwrap();
		let _: AssetDetails = tokio_test::block_on(endpoint.query(&client)).unwrap();
	}
}
