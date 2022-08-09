use da_primitives::Header;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32,
};

use crate as da_bridge;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		UpdaterManager: updater_manager::{Pallet, Call, Storage, Event<T>},
		Home: home::{Pallet, Call, Storage, Event<T>},
		DABridge: da_bridge::{Pallet, Call, Storage, Event<T>},
	}
);

frame_support::parameter_types! {
	pub const BlockHashCount: u32 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
	pub static ExistentialDeposit: u64 = 0;
}

impl system::Config for Test {
	type AccountData = ();
	type AccountId = AccountId32;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u32;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;
	type HeaderBuilder = frame_system::header_builder::da::HeaderBuilder<Test>;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type Randomness = frame_system::tests::TestRandomness<Test>;
	type SS58Prefix = ();
	type SystemWeightInfo = ();
	type Version = ();
}

impl updater_manager::Config for Test {
	type Event = Event;
}

frame_support::parameter_types! {
	pub const MaxMessageBodyBytes: u32 = 5_000;
}

impl home::Config for Test {
	type Event = Event;
	type MaxMessageBodyBytes = MaxMessageBodyBytes;
}

impl da_bridge::Config for Test {
	type Event = Event;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub(crate) fn events() -> Vec<super::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let Event::DABridge(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}
