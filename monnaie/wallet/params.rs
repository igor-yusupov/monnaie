use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Refresh {
	pub start_height: Option<u32>,
}
