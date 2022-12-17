//! Traits for HTTP REST requests.

use crate::api::{
	params::{PathParams, QueryParams},
	ApiError,
};
use bytes::Bytes;
use http::{request, response::Response, Request};
use std::{borrow::Cow, error::Error};
use url::Url;

/// An endpoint of the `cryptowat.ch` API.
/// All cryptowat.ch API Endpoints are "GET"
// Slimmed down version of Gitlab rust crates `Endpoint` trait
pub trait Endpoint {
	// we only use GET in cryptowat.ch
	// fn method(&self) -> Method;
	fn endpoint(&self) -> Cow<'static, str>;

	/// Any possible query parameters for the endpoint
	fn parameters(&self) -> QueryParams {
		QueryParams::default() // Many endpoints don't have parameters
	}

	// TODO: Maybe needed in the future, but doubt it
	// fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
	// 	Ok(None) // Many endpoints also do not have request bodies
	// }

	/// Any additional possible paths for this endpoint.
	fn path(&self) -> PathParams {
		PathParams::default() // Many endpoints do not have an additional path
	}
}

pub trait Client {
	type Error: Error + Send + Sync + 'static;
	/// Get the REST Endpoint this client interacts with
	fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError>;
	/// Execute the REST API request
	async fn rest(&self, request: request::Builder) -> Result<Response<Bytes>, ApiError>;
}

pub trait Query<T, C> {
	async fn query(&self, client: &C) -> Result<T, ApiError>;
}
