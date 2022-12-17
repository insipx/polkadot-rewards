//! "/pairs/:pair" endpoint. Lists all pairs of the assets

use crate::api::{Endpoint, PathParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct AssetPairs<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into))]
	pair: Cow<'a, str>,
}

impl<'a> AssetPairs<'a> {
	pub fn builder() -> AssetPairsBuilder<'a> {
		AssetPairsBuilder::default()
	}
}

impl<'a> Endpoint for AssetPairs<'a> {
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
		prelude::{AssetDetails, CryptowatchClient, Query, Response, RestClient},
		tests::prelude::*,
	};

	#[test]
	fn can_build() {
		AssetPairs::builder().pair("btceur").build().unwrap();
	}

	#[test]
	fn pair_is_required() {
		assert!(matches!(AssetPairs::builder().build(), Err(_)));
	}

	#[test]
	fn endpoint() {
		init();
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = AssetPairs::builder().pair("btc").build().unwrap();
		let _: Response<AssetDetails> = tokio_test::block_on(endpoint.query(&client)).unwrap();
	}
}
