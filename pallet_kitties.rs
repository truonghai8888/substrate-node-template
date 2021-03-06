
//! Autogenerated weights for `pallet_kitties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_kitties
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallet_kitties.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_kitties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_kitties::WeightInfo for WeightInfo<T> {
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Kitties Kitties (r:1 w:1)
	// Storage: Kitties CountForKitties (r:1 w:1)
	// Storage: Kitties KittiesOwned (r:1 w:1)
	fn create_kitty() -> Weight {
		(38_756_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Kitties Kitties (r:1 w:1)
	// Storage: Kitties KittiesOwned (r:2 w:2)
	fn transfer() -> Weight {
		(29_147_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
