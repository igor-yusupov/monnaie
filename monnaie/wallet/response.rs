use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Refresh {
	blocks_fetched: u32,
	received_money: bool,
}
