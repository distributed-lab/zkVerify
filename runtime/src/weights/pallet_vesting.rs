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

//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `77e87ef37af0`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-vesting
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_vesting.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.eJYai13mfL

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_vesting` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_vesting::WeightInfo for ZKVWeight<T> {
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_locked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `348 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 27_792_000 picoseconds.
        Weight::from_parts(29_089_922, 4764)
            // Standard Error: 813
            .saturating_add(Weight::from_parts(28_036, 0).saturating_mul(l.into()))
            // Standard Error: 1_447
            .saturating_add(Weight::from_parts(54_675, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_unlocked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `348 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 28_493_000 picoseconds.
        Weight::from_parts(30_086_740, 4764)
            // Standard Error: 1_158
            .saturating_add(Weight::from_parts(27_034, 0).saturating_mul(l.into()))
            // Standard Error: 2_061
            .saturating_add(Weight::from_parts(46_256, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_other_locked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 28_614_000 picoseconds.
        Weight::from_parts(30_432_752, 4764)
            // Standard Error: 1_120
            .saturating_add(Weight::from_parts(32_322, 0).saturating_mul(l.into()))
            // Standard Error: 1_994
            .saturating_add(Weight::from_parts(47_408, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[1, 28]`.
    fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 30_286_000 picoseconds.
        Weight::from_parts(31_577_260, 4764)
            // Standard Error: 842
            .saturating_add(Weight::from_parts(30_097, 0).saturating_mul(l.into()))
            // Standard Error: 1_498
            .saturating_add(Weight::from_parts(51_104, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[0, 27]`.
    fn vested_transfer(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 64_301_000 picoseconds.
        Weight::from_parts(65_343_076, 4764)
            // Standard Error: 956
            .saturating_add(Weight::from_parts(33_342, 0).saturating_mul(l.into()))
            // Standard Error: 1_701
            .saturating_add(Weight::from_parts(69_647, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[0, 27]`.
    fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `554 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `6196`
        // Minimum execution time: 65_252_000 picoseconds.
        Weight::from_parts(67_164_563, 6196)
            // Standard Error: 1_283
            .saturating_add(Weight::from_parts(23_292, 0).saturating_mul(l.into()))
            // Standard Error: 2_284
            .saturating_add(Weight::from_parts(64_458, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[2, 28]`.
    fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `449 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 29_525_000 picoseconds.
        Weight::from_parts(31_116_123, 4764)
            // Standard Error: 863
            .saturating_add(Weight::from_parts(30_945, 0).saturating_mul(l.into()))
            // Standard Error: 1_593
            .saturating_add(Weight::from_parts(51_370, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[2, 28]`.
    fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `449 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 31_269_000 picoseconds.
        Weight::from_parts(32_968_071, 4764)
            // Standard Error: 1_135
            .saturating_add(Weight::from_parts(29_673, 0).saturating_mul(l.into()))
            // Standard Error: 2_097
            .saturating_add(Weight::from_parts(52_434, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Vesting::Vesting` (r:1 w:1)
    /// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Locks` (r:1 w:1)
    /// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Freezes` (r:1 w:0)
    /// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(185), added: 2660, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// The range of component `l` is `[0, 49]`.
    /// The range of component `s` is `[2, 28]`.
    fn force_remove_vesting_schedule(l: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + l * (25 ±0) + s * (36 ±0)`
        //  Estimated: `4764`
        // Minimum execution time: 32_340_000 picoseconds.
        Weight::from_parts(33_369_558, 4764)
            // Standard Error: 969
            .saturating_add(Weight::from_parts(33_256, 0).saturating_mul(l.into()))
            // Standard Error: 1_791
            .saturating_add(Weight::from_parts(59_431, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
}
