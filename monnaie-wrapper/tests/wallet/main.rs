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

#[tokio::test]
async fn wallet_set_daemon() {
	let response = W
		.set_daemon(params::SetDaemon {
			address: Some(String::from("http://localhost:38089")),
			trusted: Some(true),
			ssl_support: None,
			ssl_private_key_path: None,
			ssl_certificate_path: None,
			ssl_ca_file: None,
			ssl_allowed_fingerprints: None,
			ssl_allow_any_cert: None,
		})
		.await;

	response.unwrap();
}

#[tokio::test]
async fn wallet_refresh() {
	let response = W.refresh(params::Refresh { start_height: None }).await;
	let response = response.unwrap();

	// assert_eq!(response.blocks_fetched, 0);
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
	// let address = W
	// 	.get_address(params::GetAddress {
	// 		account_index: 0,
	// 		address_index: None,
	// 	})
	// 	.await
	// 	.unwrap()
	// 	.address;
	// let response = W
	// 	.sweep_all(params::SweepAll {
	// 		address: address,
	// 		account_index: 0,
	// 		subaddr_indices: None,
	// 		priority: None,
	// 		mixin: 0,
	// 		ring_size: 1,
	// 		unlock_time: 0,
	// 		get_tx_keys: None,
	// 		below_amount: None,
	// 		do_not_relay: None,
	// 		get_tx_hex: None,
	// 		get_tx_metadata: None,
	// 	})
	// 	.await;
	// response.unwrap();

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
	// Выполняется долго
	// let response = W.rescan_blockchain(params::Empty { }).await;
	// response.unwrap();
}

#[tokio::test]
async fn set_tx_notes() {
	let response = W
		.set_tx_notes(params::SetTxNotes {
			txids: vec![],
			notes: vec![],
		})
		.await;
	response.unwrap();
}

#[tokio::test]
async fn get_tx_notes() {
	let response = W.get_tx_notes(params::GetTxNotes { txids: vec![] }).await;
	response.unwrap();
}

#[tokio::test]
async fn set_attribute() {
	let response = W
		.set_attribute(params::SetAttribute {
			key: String::from("key"),
			value: String::from("value"),
		})
		.await;
	response.unwrap();
}

#[tokio::test]
async fn get_attribute() {
	let _ = W
		.set_attribute(params::SetAttribute {
			key: String::from("new_key"),
			value: String::from("new_value"),
		})
		.await;

	let response = W
		.get_attribute(params::GetAttribute {
			key: String::from("new_key"),
		})
		.await;
	let response = response.unwrap();
	assert_eq!(response.value, String::from("new_value"));
}

#[tokio::test]
async fn tx_tests() {
	let new_address = W
		.create_address(params::CreateAddress {
			account_index: 0,
			label: None,
		})
		.await
		.unwrap()
		.address;

	let _ = W
		.transfer(params::Transfer {
			destinations: vec![types::Destination {
				amount: 200000000,
				address: String::from(&new_address),
			}],
			account_index: Some(0),
			subaddr_indices: None,
			priority: types::TransferPriority::Default,
			mixin: 0,
			ring_size: 7,
			unlock_time: 0,
			get_tx_key: Some(true),
			do_not_relay: None,
			get_tx_hex: Some(true),
			get_tx_metadata: None,
		})
		.await;

	let transfer = W
		.get_transfers(params::GetTransfers {
			inp: None,
			out: Some(true),
			pending: None,
			failed: None,
			pool: None,
			filter_by_height: None,
			min_height: None,
			max_height: None,
			account_index: Some(0),
			subaddr_indices: None,
		})
		.await;
	let transfer = transfer.unwrap();
	let transfer_out = transfer.out.unwrap();
	let txid = &transfer_out[transfer_out.len() - 1].txid;
	let address = &transfer_out[transfer_out.len() - 1].address;
	let response = W
		.get_tx_key(params::GetTxKey {
			txid: txid.to_string(),
		})
		.await;
	let response = response.unwrap();

	assert_eq!(response.tx_key.is_empty(), false);

	let resposne_check_tx_key = W
		.check_tx_key(params::CheckTxKey {
			txid: txid.to_string(),
			tx_key: response.tx_key,
			address: String::from(&new_address),
		})
		.await;
	resposne_check_tx_key.unwrap();
	let response_get_tx_proof = W
		.get_tx_proof(params::GetTxProof {
			txid: txid.to_string(),
			address: address.to_string(),
			message: None,
		})
		.await;

	let response_get_tx_proof = response_get_tx_proof.unwrap();

	assert_eq!(response_get_tx_proof.signature.is_empty(), false);

	let response_check_tx_proof = W
		.check_tx_proof(params::CheckTxProof {
			txid: txid.to_string(),
			address: address.to_string(),
			message: None,
			signature: (&response_get_tx_proof.signature).to_string(),
		})
		.await
		.unwrap();
	assert_eq!(response_check_tx_proof.in_pool, false);
}
