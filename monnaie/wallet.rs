use jsonrpsee::{core::client::ClientT, http_client::HttpClient, rpc_params};

mod params;
mod response;
mod types;

type Result<R> = std::result::Result<R, jsonrpsee::core::Error>;

pub struct Wallet {
	http_client: HttpClient,
}

impl Wallet {
	pub fn new(http_client: HttpClient) -> Self {
		Wallet { http_client }
	}

	pub async fn refresh(&self, start_height: Option<u32>) -> Result<response::Refresh> {
		self.http_client
			.request("refresh", rpc_params![params::Refresh { start_height }])
			.await
	}
}
