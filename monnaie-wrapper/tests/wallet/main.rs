use lazy_static::lazy_static;

mod helpers;

lazy_static! {
	static ref SHARED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);
}

#[tokio::test]
async fn get_and_set_shared_state() {
	let mut shared = SHARED.lock().unwrap();

	assert_eq!(*shared, false);

	*shared = true;
}

#[tokio::test]
async fn get_shared_state() {
	loop {
		let shared = { SHARED.lock().unwrap().clone() };

		if !shared {
			continue;
		}

		assert!(shared);

		break;
	}
}

#[tokio::test]
async fn wallet_set_daemon() {}
