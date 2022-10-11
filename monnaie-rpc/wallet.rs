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

	pub async fn get_spend_proof(
		&self,
		txid: String,
		message: Option<String>,
	) -> Result<response::GetSpendProof> {
		self.http_client
			.request(
				"get_spend_proof",
				rpc_params![params::GetSpendProof { txid, message }],
			)
			.await
	}

	pub async fn check_spend_proof(
		&self,
		txid: String,
		message: Option<String>,
		signature: String,
	) -> Result<response::CheckSpendProof> {
		self.http_client
			.request(
				"get_spend_proof",
				rpc_params![params::CheckSpendProof {
					txid,
					message,
					signature
				}],
			)
			.await
	}

	pub async fn get_reserve_proof(&self) -> Result<response::GetReserveProof> {
		self.http_client
			.request("get_reserve_proof", rpc_params!())
			.await
	}

	pub async fn make_uri(
		&self,
		address: String,
		amount: Option<usize>,
		payment_id: Option<String>,
		recipient_name: Option<String>,
		tx_description: Option<String>,
	) -> Result<response::MakeURI> {
		self.http_client
			.request(
				"make_uri",
				rpc_params!([params::MakeURI {
					address,
					amount,
					payment_id,
					recipient_name,
					tx_description,
				}]),
			)
			.await
	}
}
