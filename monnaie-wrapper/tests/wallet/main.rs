use jsonrpsee::http_client::HttpClientBuilder;
use lazy_static::lazy_static;
use monnaie_wrapper::wallet::{params, Wallet};

lazy_static! {
	static ref W: Wallet = {
		Wallet::new(
			HttpClientBuilder::default()
				.build(
					std::env::var("MONERO_WALLET_ADDRESS")
						.unwrap_or_else(|_| String::from("http://localhost:38082/json_rpc")),
				)
				.unwrap(),
		)
	};
}

#[tokio::test]
async fn wallet_refresh() {
	let response = W.refresh(params::Refresh { start_height: None }).await;

	let response = response.unwrap();

	assert_eq!(response.blocks_fetched, 0);
	assert_eq!(response.received_money, false);
}
