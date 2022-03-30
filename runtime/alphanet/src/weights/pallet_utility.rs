
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ternoa
// benchmark
// --chain
// alphanet-dev
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/alphanet/src/weights/
// --pallet=pallet_utility

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32, ) -> Weight {
		(20_477_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((3_019_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(1_520_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(10_597_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((3_176_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(8_670_000 as Weight)
	}
}