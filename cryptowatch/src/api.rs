//! Idiomatic Rust description of `cryptowat.ch` REST API.
//! This is based upon the `gitlab` rust API.
//! Described here in this [article](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)

pub mod params;
use params::QueryParams;
use std::borrow::Cow;

/// An Endpoint of the `cryptowat.ch` API.
/// All cryptowat.ch API Endpoints are "GET"
// Slimmed down version of Gitlab rust crates `Endpoint` trait
pub trait Endpoint {
	// we only use GET in cryptowat.ch
	// fn method(&self) -> Method;
	fn endpoint(&self) -> Cow<'static, str>;

	fn parameters(&self) -> QueryParams {
		QueryParams::default() // Many endpoints don't have parameters
	}

	// TODO: Maybe needed in the future, but doubt it
	// fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
	// 	Ok(None) // Many endpoints also do not have request bodies
	// }
}
