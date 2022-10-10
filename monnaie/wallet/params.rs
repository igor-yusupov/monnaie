use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Refresh {
	pub start_height: Option<u32>,
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
pub(super) struct MakeURI {
	pub address: String,
	pub amount: Option<usize>,
	pub payment_id: Option<String>,
	pub recipient_name: Option<String>,
	pub tx_description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct GetAddressBook {
	pub entries: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AddAddressBook {
	pub address: String,
	pub payment_id: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct EditAddressBook {
	pub index: usize,
	pub set_address: bool,
	pub address: Option<String>,
	pub set_description: bool,
	pub description: Option<String>,
	pub set_payment_id: bool,
	pub payment_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct DeleteAddressBook {
	pub index: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct AutoRefresh {
	pub enable: bool,
	pub period: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RescanSpent {}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CreateWallet {
	pub filename: String,
	pub password: Option<String>,
	pub language: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct OpenWallet {
	pub filename: String,
	pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CloseWallet {}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ChangeWalletPaaword {
	pub old_password: Option<String>,
	pub new_password: Option<String>,
}
