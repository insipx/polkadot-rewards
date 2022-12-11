use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("The String `{0}` is not valid in this context")]
	InvalidString(String),
}
