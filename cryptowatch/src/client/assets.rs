use super::*;
use crate::{client::CRYPTOWATCH_URI, prelude::*};

impl<'a> CryptowatchDataSet for Vec<Asset<'a>> {
	fn endpoint() -> Uri {
		CRYPTOWATCH_URI.with_query("/assets").unwrap()
	}

	async fn get() -> Result<Vec<Asset<'a>>, Error> {
		let client = RestClient::new().unwrap();
		let uri = Self::endpoint();
		// TODO: we need to split this into two functions so the user owns the data hyper returns.
		let response = client.http.get(uri).await.unwrap();
		let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
		let response: Response<Vec<Asset>> = serde_json::from_slice(&body).unwrap();
		Ok(response.unpack().into_iter().map(|a| a.into_owned()).collect::<Vec<_>>())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_asset() {
		let assets = tokio_test::block_on(Vec::<Asset>::get()).unwrap();
		assert!(assets.len() > 0);
	}
}
