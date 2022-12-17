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
