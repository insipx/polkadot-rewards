//! "/assots" endpoint. Lists all assets

use crate::api::{Endpoint, QueryParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct AssetListRequest<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into), default)]
	cursor: Option<Cow<'a, str>>,
	/// Maximum number of results to be returned (Default: 5000, Maximum: 5000)
	#[builder(default)]
	limit: Option<u64>,
}

impl<'a> AssetListRequest<'a> {
	/// Create a builder for AssetList
	pub fn builder() -> AssetListRequestBuilder<'a> {
		AssetListRequestBuilder::default()
	}
}

impl<'a> Endpoint for AssetListRequest<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		"assets".into()
	}

	fn parameters(&self) -> QueryParams {
		let mut params = QueryParams::default();

		params.push_opt("limit", self.limit).push_opt("cursor", self.cursor.as_ref());
		params
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		prelude::{Asset, CryptowatchClient, Query, RestClient},
		tests::prelude::*,
	};

	#[test]
	fn can_build() {
		AssetListRequest::builder().build().unwrap();
	}

	#[test]
	fn endpoint() {
		init();
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = AssetListRequest::builder().build().unwrap();
		let assets: Vec<Asset> = tokio_test::block_on(endpoint.query(&client)).unwrap();
		assert!(!assets.is_empty());
	}
}
