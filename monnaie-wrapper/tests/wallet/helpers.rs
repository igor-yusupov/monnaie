use monnaie_wrapper::{remote_procedure_call::HttpClientBuilder, wallet::Wallet};

pub fn create_wallet_from_env_or_default() -> Wallet {
	Wallet::new(
		HttpClientBuilder::default()
			.build(
				std::env::var("MONERO_WALLET_ADDRESS")
					.unwrap_or_else(|_| String::from("http://localhost:38089/json_rpc")),
			)
			.unwrap(),
	)
}
