use super::*;
use crate::{client::CRYPTOWATCH_URI, prelude::*};
use std::borrow::Cow;

impl<'a> CryptowatchDataSet for Vec<Asset<'a>> {
	fn endpoint(&self) -> Uri {
		todo!(); // TODO: Maybe put query data here?
	}

	async fn get_owned() -> Vec<Asset<'a>> {
		let client = RestClient::new().unwrap();
		let uri = CRYPTOWATCH_URI.with_query("/assets").unwrap();
		// we need to split this into two functions so the user owns the data hyper returns.
		let response = client.http.get(uri).await.unwrap();
		let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
		let response: Response<Vec<Asset>> = serde_json::from_slice(&body).unwrap();
		response.unpack().into_iter().map(|a| a.into_owned()).collect::<Vec<_>>()
	}

	async fn get() -> Result<hyper::Response<hyper::Body>, Error> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_asset() {
		let assets = tokio_test::block_on(Vec::<Asset>::get_owned());
		assert!(assets.len() > 0);
	}
}
