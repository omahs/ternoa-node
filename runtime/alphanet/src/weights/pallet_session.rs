
//! Autogenerated weights for `pallet_session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/ternoa
// benchmark
// pallet
// --chain=alphanet-dev
// --steps=50
// --repeat=20
// --pallet=pallet_session
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./output

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:1)
	// Storage: Session KeyOwner (r:4 w:4)
	fn set_keys() -> Weight {
		Weight::from_ref_time(172_626_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:1)
	// Storage: Session KeyOwner (r:0 w:4)
	fn purge_keys() -> Weight {
		Weight::from_ref_time(192_632_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
}
