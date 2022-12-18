//! "/assots" endpoint. Lists all assets

use crate::api::{Endpoint, QueryParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder, PartialEq)]
#[builder(setter(strip_option))]
pub struct MarketListRequest<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into), default)]
	cursor: Option<Cow<'a, str>>,
	/// Maximum number of results to be returned (Default: 5000, Maximum: 5000)
	#[builder(default)]
	limit: Option<u64>,
}

impl<'a> MarketListRequest<'a> {
	/// Create a builder for MarketList
	pub fn builder() -> MarketListRequestBuilder<'a> {
		MarketListRequestBuilder::default()
	}
}

impl<'a> Endpoint for MarketListRequest<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		"markets".into()
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
		prelude::{CryptowatchClient, CryptowatchRestClient, Market, Query},
		tests::prelude::*,
	};

	#[test]
	fn builder_is_sufficient() {
		MarketListRequest::builder().build().unwrap();
	}

	#[test]
	fn params_work() {
		let build = MarketListRequest::builder().limit(1).cursor("test").build().unwrap();
		assert_eq!(build, MarketListRequest { cursor: Some("test".into()), limit: Some(1) });
	}

	#[tokio::test]
	async fn endpoint() {
		init();
		let rest_client = CryptowatchRestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = MarketListRequest::builder().build().unwrap();
		let assets: Vec<Market> = endpoint.query(&client).await.unwrap();
		assert!(!assets.is_empty());
	}
}
