//! "/pairs/pair" endpoint. Lists details about an asset pair

use crate::api::{Endpoint, PathParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder, PartialEq)]
#[builder(setter(strip_option))]
pub struct PairDetailsRequest<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into))]
	pair: Cow<'a, str>,
}

impl<'a> PairDetailsRequest<'a> {
	pub fn builder() -> PairDetailsRequestBuilder<'a> {
		PairDetailsRequestBuilder::default()
	}
}

impl<'a> Endpoint for PairDetailsRequest<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		"pairs".into()
	}

	fn path(&self) -> PathParams {
		let mut paths = PathParams::default();
		paths.push(self.pair.as_ref());
		paths
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		prelude::{CryptowatchClient, CryptowatchRestClient, PairDetails, Query},
		tests::prelude::*,
	};

	#[test]
	fn can_build() {
		PairDetailsRequest::builder().pair("btc").build().unwrap();
	}

	#[test]
	fn pair_is_required() {
		assert!(matches!(PairDetailsRequest::builder().build(), Err(_)));
	}

	#[tokio::test]
	async fn endpoint() {
		init();
		let rest_client = CryptowatchRestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = PairDetailsRequest::builder().pair("btceur").build().unwrap();
		let _: PairDetails = endpoint.query(&client).await.unwrap();
	}
}
