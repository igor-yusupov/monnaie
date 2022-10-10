use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecureSocketsLayerSupport {
	Disabled,
	Enabled,
	Autodetect,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubaddressInformation {
	pub address_index: u32,
	pub address: String,
	pub balance: u32,
	pub unlocked_balance: u32,
	pub label: String,
	pub num_unspent_outputs: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddressInformation {
	pub address: String,
	pub label: String,
	pub address_index: u32,
	pub used: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubaddressIndex {
	pub major: u32,
	pub minor: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubaddressAccountInformation {
	pub account_index: u32,
	pub balance: u32,
	pub base_address: u32,
	pub label: Option<String>,
	pub tag: Option<String>,
	pub unlocked_balance: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountTagInformation {
	pub tag: String,
	pub label: String,
	pub accounts: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Destination {
	pub amount: u32,
	pub address: String,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum TransferPriority {
	Default,
	Unimportant,
	Normal,
	Elevated,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentInformation {
	payment_id: String,
	tx_hash: String,
	amount: u32,
	block_height: u32,
	unlock_time: u32,
	subaddr_index: SubaddressIndex,
	address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
	All,
	Available,
	Unavailable,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferInformation {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyType {
	Mnemonic,
	ViewKey,
}
