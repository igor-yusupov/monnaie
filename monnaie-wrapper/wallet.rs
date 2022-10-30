use crate::remote_procedure_call::HttpClient;

pub mod params;
pub mod result;
pub mod types;

type Result<T> = std::result::Result<T, jsonrpsee::core::Error>;

pub struct Wallet {
	http_client: HttpClient,
}

impl Wallet {
	pub fn new(http_client: HttpClient) -> Self {
		Wallet { http_client }
	}

	pub async fn set_daemon(&self, params: params::SetDaemon) -> Result<result::Empty> {
		self.http_client.request("set_daemon", Some(params)).await
	}

	pub async fn refresh(&self, params: params::Refresh) -> Result<result::Refresh> {
		self.http_client.request("refresh", Some(params)).await
	}

	pub async fn get_address(&self, params: params::GetAddress) -> Result<result::GetAddress> {
		self.http_client.request("get_address", Some(params)).await
	}

	pub async fn create_address(
		&self,
		params: params::CreateAddress,
	) -> Result<result::CreateAddress> {
		self.http_client
			.request("create_address", Some(params))
			.await
	}

	pub async fn sweep_all(&self, params: params::SweepAll) -> Result<result::SweepAll> {
		self.http_client.request("sweep_all", Some(params)).await
	}

	pub async fn sweep_single(&self, params: params::SweepSingle) -> Result<result::SweepSingle> {
		self.http_client.request("sweep_all", Some(params)).await
	}

	pub async fn relay_tx(&self, params: params::RelayTx) -> Result<result::RelayTx> {
		self.http_client.request("relay_tx", Some(params)).await
	}

	pub async fn store(&self, params: params::Empty) -> Result<result::Empty> {
		self.http_client.request("store", Some(params)).await
	}

	pub async fn get_payments(&self, params: params::GetPayments) -> Result<result::GetPayments> {
		self.http_client.request("get_payments", Some(params)).await
	}

	pub async fn get_bulk_payments(
		&self,
		params: params::GetBulkPayments,
	) -> Result<result::GetBulkPayments> {
		self.http_client
			.request("get_bulk_payments", Some(params))
			.await
	}

	pub async fn incoming_transfers(
		&self,
		params: params::IncomingTransfers,
	) -> Result<result::IncomingTransfers> {
		self.http_client
			.request("incoming_transfers", Some(params))
			.await
	}

	pub async fn query_key(&self, params: params::QueryKey) -> Result<result::QueryKey> {
		self.http_client.request("query_key", Some(params)).await
	}

	pub async fn make_integrated_address(
		&self,
		params: params::MakeIntegratedAddress,
	) -> Result<result::MakeIntegratedAddress> {
		self.http_client
			.request("make_integrated_address", Some(params))
			.await
	}

	pub async fn split_integrated_address(
		&self,
		params: params::SplitIntegratedAddress,
	) -> Result<result::SplitIntegratedAddress> {
		self.http_client
			.request("split_integrated_address", Some(params))
			.await
	}

	pub async fn stop_wallet(&self, params: params::Empty) -> Result<result::Empty> {
		self.http_client.request("stop_wallet", Some(params)).await
	}

	pub async fn rescan_blockchain(&self, params: params::Empty) -> Result<result::Empty> {
		self.http_client
			.request("rescan_blockchain", Some(params))
			.await
	}

	pub async fn set_tx_notes(&self, params: params::SetTxNotes) -> Result<result::Empty> {
		self.http_client.request("set_tx_notes", Some(params)).await
	}

	pub async fn get_tx_notes(&self, params: params::GetTxNotes) -> Result<result::GetTxNotes> {
		self.http_client.request("get_tx_notes", Some(params)).await
	}

	pub async fn set_attribute(&self, params: params::SetAttribute) -> Result<result::Empty> {
		self.http_client
			.request("set_attribute", Some(params))
			.await
	}

	pub async fn get_attribute(
		&self,
		params: params::GetAttribute,
	) -> Result<result::GetAttribute> {
		self.http_client
			.request("get_attribute", Some(params))
			.await
	}

	pub async fn get_tx_proof(&self, params: params::GetTxProof) -> Result<result::GetTxProof> {
		self.http_client.request("get_tx_proof", Some(params)).await
	}

	pub async fn check_tx_proof(
		&self,
		params: params::CheckTxProof,
	) -> Result<result::CheckTxProof> {
		self.http_client
			.request("check_tx_proof", Some(params))
			.await
	}

	pub async fn transfer(&self, params: params::Transfer) -> Result<result::Transfer> {
		self.http_client.request("transfer", Some(params)).await
	}

	pub async fn get_transfers(
		&self,
		params: params::GetTransfers,
	) -> Result<result::GetTransfers> {
		self.http_client
			.request("get_transfers", Some(params))
			.await
	}

	pub async fn get_tx_key(&self, params: params::GetTxKey) -> Result<result::GetTxKey> {
		self.http_client.request("get_tx_key", Some(params)).await
	}

	pub async fn check_tx_key(&self, params: params::CheckTxKey) -> Result<result::CheckTxKey> {
		self.http_client.request("check_tx_key", Some(params)).await
	}
}
