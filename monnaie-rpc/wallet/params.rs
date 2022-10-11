use super::types::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetSpendProof {
	pub txid: String,
	pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CheckSpendProof {
	pub txid: String,
	pub message: Option<String>,
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetReserveProof {
	pub all: bool,
	pub account_index: u32,
	pub amount: u32,
	pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CheckReserveProof {
	pub address: String,
	pub message: Option<String>,
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetTransfers {
	#[serde(rename = "in")]
	pub inp: Option<bool>,
	pub out: Option<bool>,
	pub pending: Option<bool>,
	pub failed: Option<bool>,
	pub pool: Option<bool>,
	pub filter_by_height: Option<bool>,
	pub min_height: Option<u32>,
	pub max_height: Option<u32>,
	pub account_index: Option<u32>,
	pub subaddr_indices: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetTransferByTxid {
	pub txid: String,
	pub account_index: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct DescribeTransfer {
	pub unsigned_txset: Option<String>,
	pub multisig_txset: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Sign {
	pub data: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Verify {
	pub data: String,
	pub address: String,
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ExportOutputs {
	pub all: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ImportOutputs {
	pub outputs_data_hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ExportKeyImages {
	pub all: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ImportKeyImages {
	pub signed_key_images: Vec<KeyImage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct MakeURI {
	pub address: String,
	pub amount: Option<u32>,
	pub payment_id: Option<String>,
	pub recipient_name: Option<String>,
	pub tx_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ParseURI {
	pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetAddressBook {
	pub entries: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AddAddressBook {
	pub address: String,
	pub payment_id: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct EditAddressBook {
	pub index: u32,
	pub set_address: bool,
	pub address: Option<String>,
	pub set_description: bool,
	pub description: Option<String>,
	pub set_payment_id: bool,
	pub payment_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct DeleteAddressBook {
	pub index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Refresh {
	pub start_height: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AutoRefresh {
	pub enable: bool,
	pub period: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct StartMining {
	pub threads_count: u32,
	pub do_background_mining: bool,
	pub ignore_battery: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CreateWallet {
	pub filename: String,
	pub password: Option<String>,
	pub language: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GenerateFromKeys {
	pub restore_height: i32,
	pub filename: String,
	pub address: String,
	pub spendkey: Option<String>,
	pub viewkey: String,
	pub password: String,
	pub autosave_current: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct OpenWallet {
	pub filename: String,
	pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RestoreDeterministicWallet {
	pub filename: String,
	pub password: String,
	pub seed: String,
	pub restore_height: Option<i32>,
	pub language: Option<String>,
	pub seed_offset: Option<String>,
	pub autosave_current: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ChangeWalletPaaword {
	pub old_password: Option<String>,
	pub new_password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct MakeMultisig {
	pub multisig_info: Vec<String>,
	pub threshold: u32,
	pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ImportMultisigInfo {
	pub info: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct FinalizeMultisig {
	pub multisig_info: Vec<String>,
	pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SignMultisig {
	pub tx_data_hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SubmitMultisig {
	pub tx_data_hex: String,
}
