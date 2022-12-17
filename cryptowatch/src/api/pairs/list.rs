//! "/pairs" endpoint. Lists all pairs

use crate::api::{Endpoint, QueryParams};
use derive_builder::Builder;
use std::borrow::Cow;

#[derive(Debug, Builder)]
#[builder(setter(strip_option))]
pub struct PairList<'a> {
	/// If the request is paginate, the previously received cursor value.
	#[builder(setter(into))]
	cursor: Option<Cow<'a, str>>,
	/// Maximum number of results to be returned (Default: 5000, Maximum: 5000)
	limit: Option<u64>,
}

impl<'a> PairList<'a> {
	/// Create a builder for PairList
	pub fn builder() -> PairListBuilder<'a> {
		PairListBuilder::default()
	}
}

impl<'a> Endpoint for PairList<'a> {
	fn endpoint(&self) -> Cow<'static, str> {
		"pairs".into()
	}

	fn parameters(&self) -> QueryParams {
		let mut params = QueryParams::default();

		params.push_opt("limit", self.limit).push_opt("cursor", self.cursor.as_ref());
		params
	}
}
