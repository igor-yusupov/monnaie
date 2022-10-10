use super::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Refresh {
	pub blocks_fetched: u32,
	pub received_money: bool,
}

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
pub struct GetAccountTags {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetHeight {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transfer {}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferSplit {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignTransfer {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubmitTransfer {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SweepDust {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SweepAll {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SweepSingle {}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelayTx {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetPayments {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetBulkPayments {}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomingTransfers {}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryKey {}

#[derive(Debug, Deserialize, Serialize)]
pub struct MakeIntegratedAddress {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SplitIntegratedAddress {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTxNotes {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAttribute {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTxKey {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckTxKey {}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTxProof {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckTxProof {}
