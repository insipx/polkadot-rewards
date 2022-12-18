//! "/pairs/:pair" endpoint. Lists all pairs of the assets

use crate::api::{Endpoint, PathParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder, PartialEq)]
#[builder(setter(strip_option))]
pub struct AssetDetailsRequest<'a> {
	/// If the request is paginated, the previously received cursor value.
	#[builder(setter(into))]
	pair: Cow<'a, str>,
}

impl<'a> AssetDetailsRequest<'a> {
	pub fn builder() -> AssetDetailsRequestBuilder<'a> {
		AssetDetailsRequestBuilder::default()
	}
}

impl<'a> Endpoint for AssetDetailsRequest<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		"assets".into()
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
		prelude::{AssetDetails, CryptowatchClient, CryptowatchRestClient, Query},
		tests::prelude::*,
	};

	#[test]
	fn can_build() {
		AssetDetailsRequest::builder().pair("btc").build().unwrap();
	}

	#[test]
	fn pair_is_required() {
		assert!(matches!(AssetDetailsRequest::builder().build(), Err(_)));
	}

	#[tokio::test]
	async fn endpoint() {
		init();
		let rest_client = CryptowatchRestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = AssetDetailsRequest::builder().pair("btc").build().unwrap();
		let _: AssetDetails = endpoint.query(&client).await.unwrap();
	}
}
