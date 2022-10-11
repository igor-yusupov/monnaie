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
	pub payment_id: String,
	pub tx_hash: String,
	pub amount: u32,
	pub block_height: u32,
	pub unlock_time: u32,
	pub subaddr_index: SubaddressIndex,
	pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IncomingTransferType {
	All,
	Available,
	Unavailable,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
	In,
	Out,
	Pending,
	Failed,
	Pool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferInformation {
	pub address: String,
	pub amount: u32,
	pub confirmations: u32,
	pub double_spend_seen: bool,
	pub fee: u32,
	pub height: u32,
	pub note: String,
	pub payment_id: String,
	pub subaddr_index: SubaddressIndex,
	pub suggested_confirmations_threshold: u32,
	pub timestamp: u32,
	pub txid: String,
	#[serde(rename = "type")]
	pub transfer_type: TransferType,
	pub unlock_time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SingleTransferInformation {
	pub address: String,
	pub amount: u32,
	pub amounts: Vec<u32>,
	pub confirmations: u32,
	pub destinations: Vec<Destination>,
	pub double_spend_seen: bool,
	pub fee: u32,
	pub height: u32,
	pub locked: bool,
	pub note: String,
	pub payment_id: String,
	pub subaddr_index: SubaddressIndex,
	pub suggested_confirmations_threshold: u32,
	pub timestamp: u32,
	pub txid: String,
	#[serde(rename = "type")]
	pub transfer_type: TransferType,
	pub unlock_time: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferDescription {
	pub amount_in: u32,
	pub amount_out: u32,
	pub recipients: Vec<Destination>,
	pub change_address: String,
	pub change_amount: u32,
	pub fee: u32,
	pub payment_id: String,
	pub ring_size: u32,
	pub unlock_time: u32,
	pub dummy_outputs: u32,
	pub extra: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyType {
	Mnemonic,
	ViewKey,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyImage {
	pub key_image: String,
	pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentURI {
	pub address: String,
	pub amount: u32,
	pub payment_id: Option<String>,
	pub recipient_name: String,
	pub tx_description: String,
}
