//! "/pairs/pair" endpoint. Lists details about an asset pair

use crate::api::{Endpoint, PathParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct PairDetails<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into))]
	pair: Cow<'a, str>,
}

impl<'a> PairDetails<'a> {
	pub fn builder() -> PairDetailsBuilder<'a> {
		PairDetailsBuilder::default()
	}
}

impl<'a> Endpoint for PairDetails<'a> {
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
		prelude::{CryptowatchClient, PairDetails, Query, Response, RestClient},
		tests::prelude::*,
	};

	#[test]
	fn can_build() {
		super::PairDetails::builder().pair("btc").build().unwrap();
	}

	#[test]
	fn pair_is_required() {
		assert!(matches!(super::PairDetails::builder().build(), Err(_)));
	}

	#[test]
	fn endpoint() {
		init();
		let rest_client = RestClient::with_public().unwrap();
		let client = CryptowatchClient::new_http(rest_client);
		let endpoint = super::PairDetails::builder().pair("btceur").build().unwrap();
		let _: PairDetails = tokio_test::block_on(endpoint.query(&client)).unwrap();
	}
}
