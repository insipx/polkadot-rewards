//! Mock client for testing endpoints

// TODO: Need to add the `PathParams` to things

use std::{borrow::Cow, collections::HashMap};

use bytes::Bytes;
use derive_builder::Builder;
use http::{header, request::Builder as RequestBuilder, Method, Response, StatusCode};
use serde::ser::Serialize;
use thiserror::Error;
use url::Url;

use crate::api::{ApiError, Client, RestClient};

const CLIENT_STUB: &str = "https://invalid.api.cryptowat.ch";

#[derive(Debug, Builder)]
pub struct ExpectedUrl {
	#[builder(default = "Method::GET")]
	pub method: Method,
	pub endpoint: &'static str,
	#[builder(default)]
	pub query: Vec<(Cow<'static, str>, Cow<'static, str>)>,
	#[builder(setter(strip_option, into), default)]
	pub content_type: Option<String>,
	#[builder(default)]
	pub body: Vec<u8>,
	#[builder(default = "StatusCode::OK")]
	pub status: StatusCode,

	#[builder(default = "false")]
	pub paginated: bool,
}

impl ExpectedUrlBuilder {
	pub fn add_query_params(&mut self, pairs: &[(&'static str, &'static str)]) -> &mut Self {
		self.query
			.get_or_insert_with(Vec::new)
			.extend(pairs.iter().cloned().map(|(k, v)| (k.into(), v.into())));
		self
	}

	// cryptowat.ch does not use bodies
	//pub fn body_str(&mut self, body: &str) -> &mut Self {
	//	self.body = Some(body.bytes().collect());
	//	self
	//}
}

impl ExpectedUrl {
	pub fn builder() -> ExpectedUrlBuilder {
		ExpectedUrlBuilder::default()
	}

	fn check(&self, method: Method, url: &Url) {
		// Test that the method is as expected.
		assert_eq!(method, self.method);

		// Ensure that the URL was not tampered with in the meantime.
		assert_eq!(url.scheme(), "https");
		assert_eq!(url.username(), "");
		assert_eq!(url.password(), None);
		assert_eq!(url.host_str().unwrap(), "invalid.api.cryptowat.ch");
		assert_eq!(url.port(), None);
		assert_eq!(url.path(), format!("{}", self.endpoint));
		let mut count = 0;
		for (ref key, ref value) in url.query_pairs() {
			if self.paginated && Self::is_pagination_key(key) {
				continue
			}

			let found = self
				.query
				.iter()
				.any(|(expected_key, expected_value)| key == expected_key && value == expected_value);

			if !found {
				panic!("unexpected query parameter `{}={}`", key, value);
			}
			count += 1;
		}
		assert_eq!(count, self.query.len());
		assert_eq!(url.fragment(), None);
	}

	fn is_pagination_key(key: &str) -> bool {
		key == "cursor"
	}
}

#[derive(Debug, Clone)]
struct MockResponse {
	status: StatusCode,
	data: Vec<u8>,
}

impl MockResponse {
	fn response(&self) -> Response<Vec<u8>> {
		Response::builder().status(self.status).body(self.data.clone()).unwrap()
	}
}

#[derive(Debug, Default)]
struct MockClient {
	response_map: HashMap<(Method, String), MockResponse>,
}

pub struct SingleTestClient {
	client: MockClient,
	expected: ExpectedUrl,
}

impl SingleTestClient {
	pub fn new_raw<T>(expected: ExpectedUrl, data: T) -> Self
	where
		T: Into<Vec<u8>>,
	{
		let mut client = MockClient::default();

		let request = (expected.method.clone(), format!("{}", expected.endpoint));
		let response = MockResponse { status: expected.status, data: data.into() };

		client.response_map.insert(request, response);

		Self { client, expected }
	}

	pub fn new_json<T>(expected: ExpectedUrl, data: &T) -> Self
	where
		T: Serialize,
	{
		let data = serde_json::to_vec(data).unwrap();
		Self::new_raw(expected, data)
	}
}

#[derive(Debug, Error)]
#[error("test client error")]
pub enum TestClientError {}

impl RestClient for SingleTestClient {
	type Error = TestClientError;

	fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError> {
		Ok(Url::parse(&format!("{}/{}", CLIENT_STUB, endpoint))?)
	}
}

impl Client for SingleTestClient {
	type Error = ApiError;

	async fn rest(&self, request: RequestBuilder) -> Result<Response<Bytes>, ApiError> {
		let url = Url::parse(&format!("{}", request.uri_ref().unwrap())).unwrap();
		self.expected.check(request.method_ref().unwrap().clone(), &url);

		// TODO: this is where we would check the body, if cryptowat.ch uses it in the future

		let headers = request.headers_ref().unwrap();
		let content_type = headers
			.get_all(header::CONTENT_TYPE)
			.iter()
			.map(|value| value.to_str().unwrap());
		if let Some(expected_content_type) = self.expected.content_type.as_ref() {
			itertools::assert_equal(content_type, [expected_content_type].iter().cloned());
		} else {
			assert_eq!(content_type.count(), 0);
		}

		let request = request.body(Vec::<u8>::new()).unwrap();

		Ok(self
			.client
			.response_map
			.get(&(request.method().clone(), request.uri().path().into()))
			.expect("no matching request found")
			.response()
			.map(Into::into))
	}
}
