use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Refresh {
	pub start_height: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecureSocketsLayerSupport {
	Disabled,
	Enabled,
	Autodetect,
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
pub struct SubaddressIndex {
	pub major: u32,
	pub minor: u32,
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
pub struct Destination {
	pub amount: u32,
	pub address: String,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u32)]
pub enum Priority {
	Default,
	Unimportant,
	Normal,
	Elevated,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Transfer {
	pub destinations: Vec<Destination>,
	pub account_index: Option<u32>,
	pub subaddr_indices: Option<Vec<u32>>,
	pub priority: Priority,
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
	pub priority: Priority,
	pub do_not_relay: Option<bool>,
	pub get_tx_hex: Option<bool>,
	pub new_algorithm: Option<bool>,
	pub get_tx_metadata: Option<bool>,
}
