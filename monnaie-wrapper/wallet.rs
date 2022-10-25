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

	pub async fn get_address(&self, params: params::GetAddress) -> Result<response::GetAddress> {
		self.http_client.request("get_address", Some(params)).await
	}

	pub async fn sweep_all(&self, params: params::SweepAll) -> Result<response::SweepAll> {
		self.http_client.request("sweep_all", Some(params)).await
	}

	pub async fn sweep_single(&self, params: params::SweepSingle) -> Result<response::SweepSingle> {
		self.http_client.request("sweep_all", Some(params)).await
	}

	pub async fn relay_tx(&self, params: params::RelayTx) -> Result<response::RelayTx> {
		self.http_client.request("sweep_all", Some(params)).await
	}

	pub async fn store(&self, params: params::Empty) -> Result<Empty> {
		self.http_client.request("store", Some(params)).await
	}

	pub async fn get_payments(&self, params: params::GetPayments) -> Result<response::GetPayments> {
		self.http_client.request("get_payments", Some(params)).await
	}

	pub async fn get_bulk_payments(
		&self,
		params: params::GetBulkPayments,
	) -> Result<response::GetBulkPayments> {
		self.http_client
			.request("get_bulk_payments", Some(params))
			.await
	}

	pub async fn incoming_transfers(
		&self,
		params: params::IncomingTransfers,
	) -> Result<response::IncomingTransfers> {
		self.http_client
			.request("incoming_transfers", Some(params))
			.await
	}

	pub async fn query_key(&self, params: params::QueryKey) -> Result<response::QueryKey> {
		self.http_client.request("query_key", Some(params)).await
	}

	pub async fn make_integrated_address(
		&self,
		params: params::MakeIntegratedAddress,
	) -> Result<response::MakeIntegratedAddress> {
		self.http_client
			.request("make_integrated_address", Some(params))
			.await
	}

	pub async fn split_integrated_address(
		&self,
		params: params::SplitIntegratedAddress,
	) -> Result<response::SplitIntegratedAddress> {
		self.http_client
			.request("split_integrated_address", Some(params))
			.await
	}

	pub async fn stop_wallet(&self, params: params::Empty) -> Result<Empty> {
		self.http_client.request("stop_wallet", Some(params)).await
	}
}
