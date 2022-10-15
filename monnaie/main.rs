use monnaie_wrapper::remote_procedure_call::HttpClientBuilder;
use monnaie_wrapper::wallet::Wallet;

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

	let _ = wallet;
}
