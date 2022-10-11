use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBalance {
	pub balance: u32,
	pub unlocked_balance: u32,
	pub multisig_import_needed: bool,
	pub per_subaddress: Vec<SubaddressInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAddress {
	pub address: String,
	pub addresses: Vec<AddressInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAddressIndex {
	pub index: SubaddressIndex,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAddress {
	pub address: String,
	pub address_index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateAddress {
	pub valid: bool,
	pub integrated: bool,
	pub subaddress: bool,
	pub nettype: String,
	pub openalias_address: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAccounts {
	pub subaddress_accounts: Vec<SubaddressAccountInformation>,
	pub total_balance: u32,
	pub total_unlocked_balance: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAccount {
	pub account_index: u32,
	pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAccountTags {
	pub account_tags: Vec<AccountTagInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetHeight {
	pub height: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transfer {
	pub amount: u32,
	pub fee: u32,
	pub multisig_txset: String,
	pub tx_blob: String,
	pub tx_hash: String,
	pub tx_key: String,
	pub tx_metadata: String,
	pub unsigned_txset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferSplit {
	pub tx_hash_list: Vec<String>,
	pub tx_key_list: Vec<String>,
	pub amount_list: Vec<u32>,
	pub fee_list: Vec<u32>,
	pub tx_blob_list: Vec<String>,
	pub tx_metadata_list: Vec<String>,
	pub multisig_txset: String,
	pub unsigned_txset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignTransfer {
	pub signed_txset: String,
	pub tx_hash_list: Vec<String>,
	pub tx_raw_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubmitTransfer {
	pub tx_hash_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SweepDust {
	pub tx_hash_list: Vec<String>,
	pub tx_key_list: Vec<String>,
	pub amount_list: Vec<u32>,
	pub fee_list: Vec<u32>,
	pub tx_blob_list: Vec<String>,
	pub tx_metadata_list: Vec<String>,
	pub multisig_txset: String,
	pub unsigned_txset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SweepAll {
	pub tx_hash_list: Vec<String>,
	pub tx_key_list: Vec<String>,
	pub amount_list: Vec<u32>,
	pub fee_list: Vec<u32>,
	pub tx_blob_list: Vec<String>,
	pub tx_metadata_list: Vec<String>,
	pub multisig_txset: String,
	pub unsigned_txset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SweepSingle {
	pub tx_hash_list: Vec<String>,
	pub tx_key_list: Vec<String>,
	pub amount_list: Vec<u32>,
	pub fee_list: Vec<u32>,
	pub tx_blob_list: Vec<String>,
	pub tx_metadata_list: Vec<String>,
	pub multisig_txset: String,
	pub unsigned_txset: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelayTx {
	pub tx_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPayments {
	pub payments: Vec<PaymentInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBulkPayments {
	pub payments: Vec<PaymentInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingTransfers {
	pub transfers: Vec<TransferInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryKey {
	pub key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MakeIntegratedAddress {
	pub integrated_address: String,
	pub payment_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SplitIntegratedAddress {
	pub is_subaddress: bool,
	pub payment: String,
	pub standard_address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTxNotes {
	pub notes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAttribute {
	pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTxKey {
	pub tx_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckTxKey {
	pub confirmations: u32,
	pub in_pool: bool,
	pub received: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTxProof {
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckTxProof {
	pub confirmations: u32,
	pub good: bool,
	pub in_pool: bool,
	pub received: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetSpendProof {
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckSpendProof {
	pub good: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetReserveProof {
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckReserveProof {
	pub good: bool,
	pub spent: u32,
	pub total: u32,
}

// GetTransfers

// GetTransferByTxid

// DescribeTransfer

#[derive(Debug, Deserialize, Serialize)]
pub struct Sign {
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Verify {
	pub good: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportOutputs {
	pub outputs_data_hex: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportOutputs {
	pub num_imported: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportKeyImages {
	pub signed_key_images: Vec<KeyImage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportKeyImages {
	pub height: u32,
	pub spent: u32,
	pub unspent: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MakeURI {
	pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParseURI {
	pub uri: PaymentURI,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAddressBook {
	pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddAddressBook {
	pub index: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Refresh {
	pub blocks_fetched: u32,
	pub received_money: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetLanguages {
	pub languages: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateFromKeys {
	pub address: String,
	pub info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RestoreDeterministicWallet {
	pub address: String,
	pub info: String,
	pub seed: String,
	pub was_deprecated: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IsMultisig {
	pub multisig: bool,
	pub ready: bool,
	pub threshold: u32,
	pub total: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PrepareMultisig {
	pub multisig_info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MakeMultisig {
	pub address: String,
	pub multisig_info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportMultisigInfo {
	pub info: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportMultisigInfo {
	pub n_outputs: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FinalizeMultisig {
	pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignMultisig {
	pub tx_data_hex: String,
	pub tx_hash_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubmitMultisig {
	pub tx_hash_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetVersion {
	pub version: String,
}
