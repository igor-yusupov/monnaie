use jsonrpsee::http_client::HttpClientBuilder;
use wallet::Wallet;

mod wallet;

#[tokio::main]
async fn main() {
	let matches = clap::Command::new("")
		.arg(clap::Arg::new("target").required(true))
		.get_matches();

	let wallet = Wallet::new(
		HttpClientBuilder::default()
			.build(matches.get_one::<String>("target").expect("required"))
			.unwrap(),
	);

	println!("{:?}", wallet.refresh(None).await.unwrap());
}
