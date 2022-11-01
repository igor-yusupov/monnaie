use lazy_static::lazy_static;
use monnaie_wrapper::wallet::{models, params, Wallet};

mod helpers;

lazy_static! {
	static ref W: Wallet = helpers::create_wallet_from_env_or_default();
}

#[tokio::test]
async fn set_daemon() {
	let result = W
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

	result.unwrap();
}

#[tokio::test]
async fn refresh() {
	let result = W.refresh(params::Refresh { start_height: None }).await;
	let result = result.unwrap();

	// assert_eq!(result.blocks_fetched, 0);
	assert_eq!(result.received_money, false);
}

#[tokio::test]
async fn get_address() {
	let result = W
		.get_address(params::GetAddress {
			account_index: 0,
			address_index: None,
		})
		.await;
	result.unwrap();
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
	// let result = W
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
	// result.unwrap();

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
	let result = W.store(params::Empty {}).await;
	result.unwrap();
}

#[tokio::test]
async fn get_payments() {
	let result = W
		.get_payments(params::GetPayments {
			payment_id: String::from("60900e5603bf96e3"),
		})
		.await;
	result.unwrap();
}

#[tokio::test]
async fn get_bulk_payments() {
	let result = W
		.get_bulk_payments(params::GetBulkPayments {
			payment_ids: vec![String::from("60900e5603bf96e3")],
			min_block_height: 120000,
		})
		.await;
	result.unwrap();
}

#[tokio::test]
async fn incoming_transfers() {
	let result = W
		.incoming_transfers(params::IncomingTransfers {
			transfer_type: models::IncomingTransferType::All,
			account_index: Some(0),
			subaddr_indices: None,
		})
		.await;
	let result = result.unwrap();
	assert_eq!(result.transfers.unwrap()[0].frozen, false);
}

#[tokio::test]
async fn query_key() {
	let result = W
		.query_key(params::QueryKey {
			key_type: models::KeyType::Mnemonic,
		})
		.await;
	result.unwrap();
}

#[tokio::test]
async fn make_integrated_address() {
	let result = W
		.make_integrated_address(params::MakeIntegratedAddress {
			standard_address: None,
			payment_id: None,
		})
		.await;
	let result = result.unwrap();
	assert_eq!(result.integrated_address.is_empty(), false);
	assert_eq!(result.payment_id.is_empty(), false);
}

#[tokio::test]
async fn split_integrated_address() {
	let result = W
		.make_integrated_address(params::MakeIntegratedAddress {
			standard_address: None,
			payment_id: None,
		})
		.await;
	let integrated_address = result.unwrap().integrated_address;

	let result = W
		.split_integrated_address(params::SplitIntegratedAddress {
			integrated_address: integrated_address,
		})
		.await;
	let result = result.unwrap();
	assert_eq!(result.is_subaddress, false);
	assert_eq!(result.payment_id.is_empty(), false);
	assert_eq!(result.standard_address.is_empty(), false);
}

#[tokio::test]
async fn stop_wallet() {
	// let result = W.stop_wallet(params::Empty {}).await;
	// result.unwrap();
}

#[tokio::test]
async fn rescan_blockchain() {
	// Выполняется долго
	// let result = W.rescan_blockchain(params::Empty { }).await;
	// result.unwrap();
}

#[tokio::test]
async fn set_tx_notes() {
	let result = W
		.set_tx_notes(params::SetTxNotes {
			txids: vec![],
			notes: vec![],
		})
		.await;
	result.unwrap();
}

#[tokio::test]
async fn get_tx_notes() {
	let result = W.get_tx_notes(params::GetTxNotes { txids: vec![] }).await;
	result.unwrap();
}

#[tokio::test]
async fn set_attribute() {
	let result = W
		.set_attribute(params::SetAttribute {
			key: String::from("key"),
			value: String::from("value"),
		})
		.await;
	result.unwrap();
}

#[tokio::test]
async fn get_attribute() {
	let _ = W
		.set_attribute(params::SetAttribute {
			key: String::from("new_key"),
			value: String::from("new_value"),
		})
		.await;

	let result = W
		.get_attribute(params::GetAttribute {
			key: String::from("new_key"),
		})
		.await;
	let result = result.unwrap();
	assert_eq!(result.value, String::from("new_value"));
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
			destinations: vec![models::Destination {
				amount: 200000000,
				address: String::from(&new_address),
			}],
			account_index: Some(0),
			subaddr_indices: None,
			priority: models::TransferPriority::Default,
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
	let result = W
		.get_tx_key(params::GetTxKey {
			txid: txid.to_string(),
		})
		.await;
	let result = result.unwrap();

	assert_eq!(result.tx_key.is_empty(), false);

	let resposne_check_tx_key = W
		.check_tx_key(params::CheckTxKey {
			txid: txid.to_string(),
			tx_key: result.tx_key,
			address: String::from(&new_address),
		})
		.await;
	resposne_check_tx_key.unwrap();
	let result_get_tx_proof = W
		.get_tx_proof(params::GetTxProof {
			txid: txid.to_string(),
			address: address.to_string(),
			message: None,
		})
		.await;

	let result_get_tx_proof = result_get_tx_proof.unwrap();

	assert_eq!(result_get_tx_proof.signature.is_empty(), false);

	let result_check_tx_proof = W
		.check_tx_proof(params::CheckTxProof {
			txid: txid.to_string(),
			address: address.to_string(),
			message: None,
			signature: (&result_get_tx_proof.signature).to_string(),
		})
		.await
		.unwrap();
	assert_eq!(result_check_tx_proof.in_pool, false);
}
