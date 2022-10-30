use super::models::*;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize)]
pub struct Empty {}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct SetDaemon {
	pub address: Option<String>,
	pub trusted: Option<bool>,
	pub ssl_support: Option<SecureSocketsLayerSupport>,
	pub ssl_private_key_path: Option<String>,
	pub ssl_certificate_path: Option<String>,
	pub ssl_ca_file: Option<String>,
	pub ssl_allowed_fingerprints: Option<Vec<String>>,
	pub ssl_allow_any_cert: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetBalance {
	pub account_index: u32,
	pub address_indices: Option<Vec<u32>>,
}

#[derive(Debug, Serialize)]
pub struct GetAddress {
	pub account_index: u32,
	pub address_index: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct GetAddressIndex {
	pub address: String,
}

#[derive(Debug, Serialize)]
pub struct CreateAddress {
	pub account_index: u32,
	pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LabelAddress {
	pub index: SubaddressIndex,
	pub label: String,
}

#[derive(Debug, Serialize)]
pub struct ValidateAddress {
	pub address: String,
	pub any_net_type: Option<bool>,
	pub allow_openalias: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetAccounts {
	pub tag: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateAccount {
	pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LabelAccount {
	pub account_index: u32,
	pub label: String,
}

#[derive(Debug, Serialize)]
pub struct TagAccounts {
	pub tag: String,
	pub accounts: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct UntagAccounts {
	pub accounts: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct SetAccountTagDescription {
	pub tag: String,
	pub description: String,
}

#[derive(Debug, Serialize)]
pub struct Transfer {
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

#[derive(Debug, Serialize)]
pub struct TransferSplit {
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

#[derive(Debug, Serialize)]
pub struct SignTransfer {
	pub unsigned_txset: String,
	pub export_raw: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SubmitTransfer {
	pub tx_data_hex: String,
}

#[derive(Debug, Serialize)]
pub struct SweepDust {
	pub get_tx_keys: Option<bool>,
	pub do_not_relay: Option<bool>,
	pub get_tx_hex: Option<bool>,
	pub get_tx_metadata: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SweepAll {
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

#[derive(Debug, Serialize)]
pub struct SweepSingle {
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

#[derive(Debug, Serialize)]
pub struct RelayTx {
	pub hex: String,
}

#[derive(Debug, Serialize)]
pub struct GetPayments {
	pub payment_id: String,
}

#[derive(Debug, Serialize)]
pub struct GetBulkPayments {
	pub payment_ids: Vec<String>,
	pub min_block_height: u32,
}

#[derive(Debug, Serialize)]
pub struct IncomingTransfers {
	pub transfer_type: IncomingTransferType,
	pub account_index: Option<u32>,
	pub subaddr_indices: Option<Vec<u32>>,
}

#[derive(Debug, Serialize)]
pub struct QueryKey {
	pub key_type: KeyType,
}

#[derive(Debug, Serialize)]
pub struct MakeIntegratedAddress {
	pub standard_address: Option<String>,
	pub payment_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SplitIntegratedAddress {
	pub integrated_address: String,
}

#[derive(Debug, Serialize)]
pub struct SetTxNotes {
	pub txids: Vec<String>,
	pub notes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct GetTxNotes {
	pub txids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SetAttribute {
	pub key: String,
	pub value: String,
}

#[derive(Debug, Serialize)]
pub struct GetAttribute {
	pub key: String,
}

#[derive(Debug, Serialize)]
pub struct GetTxKey {
	pub txid: String,
}

#[derive(Debug, Serialize)]
pub struct CheckTxKey {
	pub txid: String,
	pub tx_key: String,
	pub address: String,
}

#[derive(Debug, Serialize)]
pub struct GetTxProof {
	pub txid: String,
	pub address: String,
	pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckTxProof {
	pub txid: String,
	pub address: String,
	pub message: Option<String>,
	pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct GetSpendProof {
	pub txid: String,
	pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckSpendProof {
	pub txid: String,
	pub message: Option<String>,
	pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct GetReserveProof {
	pub all: bool,
	pub account_index: u32,
	pub amount: u64,
	pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckReserveProof {
	pub address: String,
	pub message: Option<String>,
	pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct GetTransfers {
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

#[derive(Debug, Serialize)]
pub struct GetTransferByTxid {
	pub txid: String,
	pub account_index: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct DescribeTransfer {
	pub unsigned_txset: Option<String>,
	pub multisig_txset: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Sign {
	pub data: String,
}

#[derive(Debug, Serialize)]
pub struct Verify {
	pub data: String,
	pub address: String,
	pub signature: String,
}

#[derive(Debug, Serialize)]
pub struct ExportOutputs {
	pub all: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ImportOutputs {
	pub outputs_data_hex: String,
}

#[derive(Debug, Serialize)]
pub struct ExportKeyImages {
	pub all: bool,
}

#[derive(Debug, Serialize)]
pub struct ImportKeyImages {
	pub signed_key_images: Vec<KeyImage>,
}

#[derive(Debug, Serialize)]
pub struct MakeURI {
	pub address: String,
	pub amount: Option<u32>,
	pub payment_id: Option<String>,
	pub recipient_name: Option<String>,
	pub tx_description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ParseURI {
	pub uri: String,
}

#[derive(Debug, Serialize)]
pub struct GetAddressBook {
	pub entries: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct AddAddressBook {
	pub address: String,
	pub payment_id: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EditAddressBook {
	pub index: u32,
	pub set_address: bool,
	pub address: Option<String>,
	pub set_description: bool,
	pub description: Option<String>,
	pub set_payment_id: bool,
	pub payment_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DeleteAddressBook {
	pub index: u32,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
pub struct Refresh {
	pub start_height: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct AutoRefresh {
	pub enable: bool,
	pub period: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct StartMining {
	pub threads_count: u32,
	pub do_background_mining: bool,
	pub ignore_battery: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateWallet {
	pub filename: String,
	pub password: Option<String>,
	pub language: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateFromKeys {
	pub restore_height: i32,
	pub filename: String,
	pub address: String,
	pub spendkey: Option<String>,
	pub viewkey: String,
	pub password: String,
	pub autosave_current: bool,
}

#[derive(Debug, Serialize)]
pub struct OpenWallet {
	pub filename: String,
	pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RestoreDeterministicWallet {
	pub filename: String,
	pub password: String,
	pub seed: String,
	pub restore_height: Option<i32>,
	pub language: Option<String>,
	pub seed_offset: Option<String>,
	pub autosave_current: bool,
}

#[derive(Debug, Serialize)]
pub struct ChangeWalletPaaword {
	pub old_password: Option<String>,
	pub new_password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MakeMultisig {
	pub multisig_info: Vec<String>,
	pub threshold: u32,
	pub password: String,
}

#[derive(Debug, Serialize)]
pub struct ImportMultisigInfo {
	pub info: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FinalizeMultisig {
	pub multisig_info: Vec<String>,
	pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignMultisig {
	pub tx_data_hex: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitMultisig {
	pub tx_data_hex: String,
}
