use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Refresh {
	pub start_height: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SetDaemon {
	pub address: Option<String>,
	pub trusted: Option<bool>,
	pub ssl_support: Option<SecureSocketsLayerSupport>,
	pub ssl_private_key_path: Option<String>,
	pub ssl_certificate_path: Option<String>,
	pub ssl_ca_file: Option<String>,
	pub ssl_allowed_fingerprints: Option<Vec<String>>,
	pub ssl_allow_any_cert: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetBalance {
	pub account_index: u32,
	pub address_indices: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetAddress {
	pub account_index: u32,
	pub address_index: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetAddressIndex {
	pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CreateAddress {
	pub account_index: u32,
	pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct LabelAddress {
	pub index: SubaddressIndex,
	pub label: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ValidateAddress {
	pub address: String,
	pub any_net_type: Option<bool>,
	pub allow_openalias: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetAccounts {
	pub tag: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CreateAccount {
	pub label: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct LabelAccount {
	pub account_index: u32,
	pub label: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct TagAccounts {
	pub tag: String,
	pub accounts: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct UntagAccounts {
	pub accounts: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SetAccountTagDescription {
	pub tag: String,
	pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Transfer {
	pub destinations: Vec<Destination>,
	pub account_index: Option<u32>,
	pub subaddr_indices: Option<Vec<u32>>,
	pub priority: TransferPriority,
	pub mixin: u32,
	pub ring_size: u32,
	pub unlock_time: u32,
	pub get_tx_key: Option<bool>,
	pub do_not_relay: Option<bool>,
	pub get_tx_hex: Option<bool>,
	pub get_tx_metadata: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct TransferSplit {
	pub destinations: Vec<Destination>,
	pub account_index: Option<u32>,
	pub subaddr_indices: Option<Vec<u32>>,
	pub mixin: u32,
	pub ring_size: u32,
	pub unlock_time: u32,
	pub get_tx_keys: Option<bool>,
	pub priority: TransferPriority,
	pub do_not_relay: Option<bool>,
	pub get_tx_hex: Option<bool>,
	pub new_algorithm: Option<bool>,
	pub get_tx_metadata: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SignTransfer {
	pub unsigned_txset: String,
	pub export_raw: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SubmitTransfer {
	pub tx_data_hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SweepDust {
	pub get_tx_keys: Option<bool>,
	pub do_not_relay: Option<bool>,
	pub get_tx_hex: Option<bool>,
	pub get_tx_metadata: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SweepAll {
	pub address: String,
	pub account_index: u32,
	pub subaddr_indices: Option<Vec<u32>>,
	pub priority: Option<u32>,
	pub mixin: u32,
	pub ring_size: u32,
	pub unlock_time: u32,
	pub get_tx_keys: Option<bool>,
	pub below_amount: Option<u32>,
	pub do_not_relay: Option<bool>,
	pub get_tx_hex: Option<bool>,
	pub get_tx_metadata: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SweepSingle {
	pub address: String,
	pub account_index: u32,
	pub subaddr_indices: Option<Vec<u32>>,
	pub priority: Option<u32>,
	pub mixin: u32,
	pub ring_size: u32,
	pub unlock_time: u32,
	pub get_tx_keys: Option<bool>,
	pub key_image: String,
	pub below_amount: Option<u32>,
	pub do_not_relay: Option<bool>,
	pub get_tx_hex: Option<bool>,
	pub get_tx_metadata: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RelayTx {
	pub hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetPayments {
	pub payment_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetBulkPayments {
	pub payment_ids: Vec<String>,
	pub min_block_height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct IncomingTransfers {
	pub transfer_type: TransferType,
	pub account_index: Option<u32>,
	pub subaddr_indices: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct QueryKey {
	pub key_type: KeyType,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct MakeIntegratedAddress {
	pub standard_address: Option<String>,
	pub payment_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SplitIntegratedAddress {
	pub integrated_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SetTxNotes {
	pub txids: Vec<String>,
	pub notes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetTxNotes {
	pub txids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SetAttribute {
	pub key: String,
	pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetAttribute {
	pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetTxKey {
	pub txid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CheckTxKey {
	pub txid: String,
	pub tx_key: String,
	pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetTxProof {
	pub txid: String,
	pub address: String,
	pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CheckTxProof {
	pub txid: String,
	pub address: String,
	pub message: Option<String>,
	pub signature: String,
}
