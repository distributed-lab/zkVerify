// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_aggregate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2025-02-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-aggregate
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/mdamico/devel/zkVerify/HEADER-APACHE2
// --output
// pallets/aggregate/src/weight.rs
// --template
// /home/mdamico/devel/zkVerify/node/zkv-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_aggregate`.
pub trait WeightInfo {
    fn on_proof_verified() -> Weight;
    fn aggregate(n: u32, ) -> Weight;
    fn aggregate_on_invalid_domain() -> Weight;
    fn aggregate_on_invalid_id() -> Weight;
    fn register_domain() -> Weight;
    fn hold_domain() -> Weight;
    fn unregister_domain() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    fn on_proof_verified() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1540`
        //  Estimated: `177995`
        // Minimum execution time: 54_020_000 picoseconds.
        Weight::from_parts(56_093_000, 177995)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Published` (r:1 w:1)
    /// Proof: `Aggregate::Published` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 128]`.
    fn aggregate(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `294 + n * (80 ±0)`
        //  Estimated: `177995 + n * (80 ±0)`
        // Minimum execution time: 46_600_000 picoseconds.
        Weight::from_parts(19_953_343, 177995)
            // Standard Error: 22_339
            .saturating_add(Weight::from_parts(26_040_026, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 80).saturating_mul(n.into()))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:0)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    fn aggregate_on_invalid_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `177995`
        // Minimum execution time: 6_289_000 picoseconds.
        Weight::from_parts(6_483_000, 177995)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:0)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    fn aggregate_on_invalid_id() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `217`
        //  Estimated: `177995`
        // Minimum execution time: 7_554_000 picoseconds.
        Weight::from_parts(7_821_000, 177995)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    /// Storage: `Aggregate::NextDomainId` (r:1 w:1)
    /// Proof: `Aggregate::NextDomainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:0 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    fn register_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `3586`
        // Minimum execution time: 43_509_000 picoseconds.
        Weight::from_parts(44_821_000, 3586)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    fn hold_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `217`
        //  Estimated: `177995`
        // Minimum execution time: 10_989_000 picoseconds.
        Weight::from_parts(11_319_000, 177995)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    fn unregister_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `217`
        //  Estimated: `177995`
        // Minimum execution time: 11_083_000 picoseconds.
        Weight::from_parts(11_738_000, 177995)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}