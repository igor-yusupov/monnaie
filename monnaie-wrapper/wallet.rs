use serde::Deserialize;

use crate::remote_procedure_call::HttpClient;

pub mod params;
pub mod response;
pub mod types;

type Result<T> = std::result::Result<T, jsonrpsee::core::Error>;

pub struct Wallet {
	http_client: HttpClient,
}

#[derive(Deserialize)]
pub struct Empty {}

impl Wallet {
	pub fn new(http_client: HttpClient) -> Self {
		Wallet { http_client }
	}

	pub async fn set_daemon(&self, params: params::SetDaemon) -> Result<Empty> {
		self.http_client.request("set_daemon", Some(params)).await
	}

	pub async fn refresh(&self, params: params::Refresh) -> Result<response::Refresh> {
		self.http_client.request("refresh", Some(params)).await
	}
}
