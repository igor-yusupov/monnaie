use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Refresh {
	blocks_fetched: u32,
	received_money: bool,
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
	pub all: bool,
	pub account_index: usize,
	pub amount: usize,
	pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MakeURI {
	pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAddressBook {
	pub uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddAddressBook {
	pub index: usize,
}
