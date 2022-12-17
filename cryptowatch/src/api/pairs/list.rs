//! "/pairs" endpoint. Lists all pairs

use crate::api::{Endpoint, QueryParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct PairListRequest<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into), default)]
	cursor: Option<Cow<'a, str>>,
	/// Maximum number of results to be returned (Default: 5000, Maximum: 5000)
	#[builder(default)]
	limit: Option<u64>,
}

impl<'a> PairListRequest<'a> {
	/// Create a builder for PairList
	pub fn builder() -> PairListRequestBuilder<'a> {
		PairListRequestBuilder::default()
	}
}

impl<'a> Endpoint for PairListRequest<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		"pairs".into()
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
		prelude::{CryptowatchClient, PairInfo, Query, RestClient},
		tests::prelude::*,
	};

	#[test]
	fn can_build() {
		PairListRequest::builder().cursor("test").limit(5).build().unwrap();
	}

	#[test]
	fn does_not_require_params() {
		assert_ok!(PairListRequest::builder().build());
	}

	#[test]
	fn endpoint() {
		init();
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = PairListRequest::builder().build().unwrap();
		let _: Vec<PairInfo> = tokio_test::block_on(endpoint.query(&client)).unwrap();
	}
}
