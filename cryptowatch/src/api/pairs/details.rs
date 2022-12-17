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
		"pairs/".into()
	}

	fn path(&self) -> PathParams {
		let mut paths = PathParams::default();
		paths.push(self.pair.as_ref());
		paths
	}
}
