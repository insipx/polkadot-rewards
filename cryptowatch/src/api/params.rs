use crate::api::ApiError;
use chrono::{DateTime, NaiveDate, Utc};
use std::borrow::Cow;
use url::Url;

/// A trait representing a parameter value.
pub trait ParamValue<'a> {
	#[allow(clippy::wrong_self_convention)]
	/// The parameter value as a string.
	fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
	fn as_value(&self) -> Cow<'static, str> {
		if *self {
			"true".into()
		} else {
			"false".into()
		}
	}
}

impl<'a> ParamValue<'a> for &'a str {
	fn as_value(&self) -> Cow<'a, str> {
		(*self).into()
	}
}

impl ParamValue<'static> for String {
	fn as_value(&self) -> Cow<'static, str> {
		self.clone().into()
	}
}

impl<'a> ParamValue<'a> for &'a String {
	fn as_value(&self) -> Cow<'a, str> {
		(*self).into()
	}
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
	fn as_value(&self) -> Cow<'a, str> {
		self.clone()
	}
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
	fn as_value(&self) -> Cow<'a, str> {
		(*self).clone()
	}
}

impl ParamValue<'static> for u64 {
	fn as_value(&self) -> Cow<'static, str> {
		format!("{self}").into()
	}
}

impl ParamValue<'static> for f64 {
	fn as_value(&self) -> Cow<'static, str> {
		format!("{self}").into()
	}
}

impl ParamValue<'static> for DateTime<Utc> {
	fn as_value(&self) -> Cow<'static, str> {
		self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true).into()
	}
}

impl ParamValue<'static> for NaiveDate {
	fn as_value(&self) -> Cow<'static, str> {
		format!("{}", self.format("%Y-%m-%d")).into()
	}
}

/// A structure for paramaters that add to a path
#[derive(Debug, Default, Clone)]
pub struct PathParams<'a> {
	paths: Vec<Cow<'a, str>>,
}

impl<'a> PathParams<'a> {
	/// Push one path to the parameters.
	pub fn push<P>(&mut self, path: P) -> &mut Self
	where
		P: Into<Cow<'a, str>>,
	{
		self.paths.push(path.into());
		self
	}

	/// Push a single path.
	pub fn push_opt<'b, P>(&mut self, path: Option<P>) -> &mut Self
	where
		P: ParamValue<'a>,
		'b: 'a,
	{
		if let Some(path) = path {
			self.paths.push(path.as_value());
		}
		self
	}

	/// Extend parameters with many paths.
	pub fn extend<P, I>(&mut self, iter: I) -> &mut Self
	where
		P: Into<Cow<'a, str>>,
		I: Iterator<Item = P>,
	{
		self.paths.extend(iter.map(Into::into));
		self
	}

	pub fn add_to_url(&self, url: &mut Url) -> Result<(), ApiError> {
		let mut paths = url.path_segments_mut().map_err(|_| ApiError::CannotBeABase)?;
		paths.extend(self.paths.iter());
		Ok(())
	}
}

/// A structure for query parameters.
#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
	params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
	/// Push a single parameter.
	pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
	where
		K: Into<Cow<'a, str>>,
		V: ParamValue<'b>,
		'b: 'a,
	{
		self.params.push((key.into(), value.as_value()));
		self
	}

	/// Push a single parameter.
	pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
	where
		K: Into<Cow<'a, str>>,
		V: ParamValue<'b>,
		'b: 'a,
	{
		if let Some(value) = value {
			self.params.push((key.into(), value.as_value()));
		}
		self
	}

	/// Push a set of parameters.
	pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
	where
		I: Iterator<Item = (K, V)>,
		K: Into<Cow<'a, str>>,
		V: ParamValue<'b>,
		'b: 'a,
	{
		self.params.extend(iter.map(|(key, value)| (key.into(), value.as_value())));
		self
	}

	/// Add the parameters to a URL.
	pub fn add_to_url(&self, url: &mut Url) {
		let mut pairs = url.query_pairs_mut();
		pairs.extend_pairs(self.params.iter());
	}
}
