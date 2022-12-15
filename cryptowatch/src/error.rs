use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("The String `{0}` is not valid in this context")]
	InvalidString(String),
	#[error(
		"The exchange `{0}` is not currently supported. Please open an issue on GitHub if you think it should be."
	)]
	UnsupportedExchange(String),
	#[error(transparent)]
	IO(#[from] std::io::Error),
	#[error("The header `{0}` is invalid")]
	InvalidHeader(#[from] hyper::header::InvalidHeaderValue),
	#[error(transparent)]
	HttpError(#[from] hyper::http::Error),
	#[error("The authority is missing from the URI")]
	NoAuthority,
	#[error("The scheme is missing from the URI")]
	NoScheme,
}
