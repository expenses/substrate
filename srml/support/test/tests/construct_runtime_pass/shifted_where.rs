#[path = "../system.rs"]
mod system;

use support::sr_primitives::generic;
use support::sr_primitives::traits::{BlakeTwo256, Block as _, Verify};
use primitives::{H256, sr25519};

pub type Signature = sr25519::Signature;
pub type AccountId = <Signature as Verify>::Signer;
pub type BlockNumber = u64;
pub type Index = u64;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<u32, Call, Signature, ()>;

impl system::Trait for Runtime {
	type Hash = H256;
	type Origin = Origin;
	type BlockNumber = BlockNumber;
	type AccountId = AccountId;
	type Event = Event;
}

support::construct_runtime!(
	pub enum Runtime where
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
		Block = Block
	{
		System: system::{Module, Call, Event},
	}
);

fn main() {}
