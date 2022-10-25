use lazy_static::lazy_static;
use monnaie_wrapper::remote_procedure_call::HttpClientBuilder;
use monnaie_wrapper::wallet::types;
use monnaie_wrapper::wallet::{params, Wallet};

lazy_static! {
	static ref W: Wallet = {
		Wallet::new(
			HttpClientBuilder::default()
				.build(
					std::env::var("MONERO_WALLET_ADDRESS")
						.unwrap_or_else(|_| String::from("http://localhost:38089/json_rpc")),
				)
				.unwrap(),
		)
	};
}

// #[tokio::test]
// async fn wallet_set_daemon() {
// 	let response = W
// 		.set_daemon(params::SetDaemon {
// 			address: Some(String::from("http://localhost:38089")),
// 			trusted: Some(true),
// 			ssl_support: None,
// 			ssl_private_key_path: None,
// 			ssl_certificate_path: None,
// 			ssl_ca_file: None,
// 			ssl_allowed_fingerprints: None,
// 			ssl_allow_any_cert: None,
// 		})
// 		.await;

// 	let _ = response.unwrap();
// }

#[tokio::test]
async fn wallet_refresh() {
	let response = W.refresh(params::Refresh { start_height: None }).await;
	let response = response.unwrap();

	assert_eq!(response.blocks_fetched, 0);
	assert_eq!(response.received_money, false);
}

#[tokio::test]
async fn get_address() {
	let response = W
		.get_address(params::GetAddress {
			account_index: 0,
			address_index: None,
		})
		.await;
	response.unwrap();
}

#[tokio::test]
async fn sweep_all() {
	// let address = W.get_address(params::GetAddress { account_index: 0, address_index: None }).await.unwrap().address;
	// let response = W.sweep_all(params::SweepAll {
	// 	address: address,
	// 	account_index: 0,
	// 	subaddr_indices: None,
	// 	priority: None,
	// 	mixin: 0,
	// 	ring_size: 1,
	// 	unlock_time: 0,
	// 	get_tx_keys: None,
	// 	below_amount: None,
	// 	do_not_relay: None,
	// 	get_tx_hex: None,
	// 	get_tx_metadata: None,
	// }).await;
	// let response = response.unwrap();

	// TODO: выдает ошибку No unlocked balance in the specified account
}

#[tokio::test]
async fn sweep_single() {
	// TODO: аналогично предыдущему
}

#[tokio::test]
async fn relay_tx() {
	// TODO: непонятно пока откуда брать hex
}

#[tokio::test]
async fn store() {
	let response = W.store(params::Empty {}).await;
	response.unwrap();
}

#[tokio::test]
async fn get_payments() {
	let response = W
		.get_payments(params::GetPayments {
			payment_id: String::from("60900e5603bf96e3"),
		})
		.await;
	response.unwrap();
}

#[tokio::test]
async fn get_bulk_payments() {
	let response = W
		.get_bulk_payments(params::GetBulkPayments {
			payment_ids: vec![String::from("60900e5603bf96e3")],
			min_block_height: 120000,
		})
		.await;
	response.unwrap();
}

#[tokio::test]
async fn incoming_transfers() {
	let response = W
		.incoming_transfers(params::IncomingTransfers {
			transfer_type: types::IncomingTransferType::All,
			account_index: Some(0),
			subaddr_indices: None,
		})
		.await;
	let response = response.unwrap();
	assert_eq!(response.transfers.unwrap()[0].frozen, false);
}

#[tokio::test]
async fn query_key() {
	let response = W
		.query_key(params::QueryKey {
			key_type: types::KeyType::Mnemonic,
		})
		.await;
	response.unwrap();
}

#[tokio::test]
async fn make_integrated_address() {
	let response = W
		.make_integrated_address(params::MakeIntegratedAddress {
			standard_address: None,
			payment_id: None,
		})
		.await;
	let response = response.unwrap();
	assert_eq!(response.integrated_address.is_empty(), false);
	assert_eq!(response.payment_id.is_empty(), false);
}

#[tokio::test]
async fn split_integrated_address() {
	let response = W
		.make_integrated_address(params::MakeIntegratedAddress {
			standard_address: None,
			payment_id: None,
		})
		.await;
	let integrated_address = response.unwrap().integrated_address;

	let response = W
		.split_integrated_address(params::SplitIntegratedAddress {
			integrated_address: integrated_address,
		})
		.await;
	let response = response.unwrap();
	assert_eq!(response.is_subaddress, false);
	assert_eq!(response.payment_id.is_empty(), false);
	assert_eq!(response.standard_address.is_empty(), false);
}

#[tokio::test]
async fn stop_wallet() {
	// let response = W.stop_wallet(params::Empty {}).await;
	// response.unwrap();
}

#[tokio::test]
async fn rescan_blockchain() {
	// TODO
}

#[tokio::test]
async fn set_tx_notes() {
	// TODO
}

#[tokio::test]
async fn get_tx_notes() {
	// TODO
}

#[tokio::test]
async fn set_attribute() {
	// TODO
}

#[tokio::test]
async fn get_attribute() {
	// TODO
}

#[tokio::test]
async fn get_tx_key() {
	// TODO
}

#[tokio::test]
async fn check_tx_key() {
	// TODO
}

#[tokio::test]
async fn get_tx_proof() {
	// TODO
}

#[tokio::test]
async fn check_tx_proof() {
	// TODO
}
