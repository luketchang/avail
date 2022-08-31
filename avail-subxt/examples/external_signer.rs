use avail_subxt::{avail::runtime_types::frame_support::storage::bounded_vec::BoundedVec, *};
use sp_keyring::AccountKeyring;
use subxt::{
	ext::sp_core::{ecdsa::Pair, Pair as PairT},
	tx::PairSigner,
	OnlineClient,
};

/// This example submits an Avail data extrinsic, then retrieves the block containing the
/// extrinsic and matches the data.
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let api = OnlineClient::<AvailConfig>::from_url("ws://127.0.0.1:9944").await?;

	let pair = Pair::from_string(
		"0x1111111111111111111111111111111111111111111111111111111111111111",
		None,
	)
	.unwrap();
	let signer = PairSigner::<AvailConfig, _>::new(pair);
	println!("Tx signer account id: {}", signer.account_id());

	// Alice sends funds to 0x1111
	let alice = PairSigner::new(AccountKeyring::Alice.pair());
	let alice_id = AccountKeyring::Alice.to_account_id();
	let money_transfer = avail::tx()
		.balances()
		.transfer(signer.account_id().clone().into(), 50000 as u128);
	println!("Sending Alice funds to other signer...");
	let h = api
		.tx()
		.sign_and_submit_then_watch(&money_transfer, &alice, Default::default())
		.await
		.unwrap()
		.wait_for_finalized_success()
		.await
		.unwrap();
	let submitted_block = api
		.rpc()
		.block(Some(h.block_hash()))
		.await
		.unwrap()
		.unwrap();

	let xts = submitted_block.block.extrinsics;
	println!("Submitted block extrinsic: {xts:?}");

	let example_data = b"example".to_vec();
	let data_transfer = avail::tx()
		.data_availability()
		.submit_data(BoundedVec(example_data.clone()));
	let extrinsic_params = AvailExtrinsicParams::new_with_app_id(1);
	println!("Sending example data...");
	let h = api
		.tx()
		.sign_and_submit_then_watch(&data_transfer, &signer, extrinsic_params)
		.await
		.unwrap()
		.wait_for_finalized_success()
		.await
		.unwrap();

	let submitted_block = api
		.rpc()
		.block(Some(h.block_hash()))
		.await
		.unwrap()
		.unwrap();
	let xts = submitted_block.block.extrinsics;
	println!("Submitted block extrinsic: {xts:?}");

	Ok(())
}
