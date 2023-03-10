// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_club
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-01, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `pokeball`, CPU: `Intel(R) Core(TM) i5-8250U CPU @ 1.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_club
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --template
// ./frame-weight-template.hbs
// --output
// ./pallets/club/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_club.
pub trait WeightInfo {
	fn add_new_club() -> Weight;
	fn add_member() -> Weight;
	fn remove_club(s: u32, ) -> Weight;
	fn remove_member() -> Weight;
}

/// Weights for pallet_club using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ClubModule Clubs (r:1 w:1)
	fn add_new_club() -> Weight {
		(27_602_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ClubModule Clubs (r:1 w:1)
	// Storage: ClubModule Membership (r:1 w:1)
	fn add_member() -> Weight {
		(39_023_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: ClubModule Clubs (r:1 w:1)
	// Storage: ClubModule Membership (r:0 w:5000)
	/// The range of component `s` is `[0, 100000]`.
	fn remove_club(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 154_000
			.saturating_add((3_385_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: ClubModule Clubs (r:1 w:1)
	// Storage: ClubModule Membership (r:1 w:1)
	fn remove_member() -> Weight {
		(37_575_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ClubModule Clubs (r:1 w:1)
	fn add_new_club() -> Weight {
		(27_602_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ClubModule Clubs (r:1 w:1)
	// Storage: ClubModule Membership (r:1 w:1)
	fn add_member() -> Weight {
		(39_023_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: ClubModule Clubs (r:1 w:1)
	// Storage: ClubModule Membership (r:0 w:5000)
	/// The range of component `s` is `[0, 100000]`.
	fn remove_club(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 154_000
			.saturating_add((3_385_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	// Storage: ClubModule Clubs (r:1 w:1)
	// Storage: ClubModule Membership (r:1 w:1)
	fn remove_member() -> Weight {
		(37_575_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
