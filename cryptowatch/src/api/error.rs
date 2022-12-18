use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
	#[error("URL cannot be a base")]
	CannotBeABase,
	#[error("HTTP Error Originating from cryptowat.ch `{}`", .0)]
	HttpError(http::StatusCode),
	#[error("An internal error occurred on cryptowat.ch servers `{}`", status)]
	CryptowatchService { status: http::StatusCode, data: Vec<u8> },
	#[error("Could not parse {} data from JSON: {}", name, source)]
	DataType { source: serde_json::Error, name: &'static str },
	#[error("The endpoint `{}` does not create a valid url with `{}`, {}", endpoint, url, source)]
	InvalidUrl { url: url::Url, endpoint: String, source: url::ParseError },
	#[error(transparent)]
	REST(#[from] hyper::Error),
	#[error("{}: {}", .0, .0.to_string())]
	Http(#[from] http::Error),

	#[cfg(test)]
	#[error(transparent)]
	UrlParse(#[from] url::ParseError),
}

impl ApiError {
	pub(crate) fn server_error(status: http::StatusCode, body: &bytes::Bytes) -> Self {
		Self::CryptowatchService { status, data: body.into_iter().copied().collect() }
	}

	pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
		ApiError::DataType { source, name: std::any::type_name::<T>() }
	}

	pub(crate) fn invalid_url_endpoint(endpoint: &str, source: url::ParseError) -> Self {
		ApiError::InvalidUrl { url: crate::client::CRYPTOWATCH_URL.clone(), endpoint: endpoint.into(), source }
	}
}
