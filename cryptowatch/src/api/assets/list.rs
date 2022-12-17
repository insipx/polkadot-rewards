//! "/assets" endpoint. Lists all assets

use crate::api::{Endpoint, QueryParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct AssetList<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into))]
	cursor: Option<Cow<'a, str>>,
	/// Maximum number of results to be returned (Default: 5000, Maximum: 5000)
	limit: Option<u64>,
}

impl<'a> AssetList<'a> {
	/// Create a builder for AssetList
	pub fn builder() -> AssetListBuilder<'a> {
		AssetListBuilder::default()
	}
}

impl<'a> Endpoint for AssetList<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		format!("assets/").into()
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
	use crate::prelude::{Asset, CryptowatchClient, Query, RestClient};

	#[test]
	fn endpoint() {
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = AssetList::builder().build().unwrap();
		let assets: Vec<Asset> = tokio_test::block_on(endpoint.query(&client)).unwrap();
		assert!(assets.len() > 0);
	}
}
