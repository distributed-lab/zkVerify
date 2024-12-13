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

//! Autogenerated weights for `pallet_hyperbridge_aggregations`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `LAPTOP-5V1NHBSA`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /home/danielecker/hl-crypto/zkVerify/target/production/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-hyperbridge-aggregations
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/danielecker/hl-crypto/zkVerify/HEADER-APACHE2
// --output
// pallets/hyperbridge_aggregations/src/weight.rs
// --template
// /home/danielecker/hl-crypto/zkVerify/node/zkv-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_hyperbridge_aggregations`.
pub trait WeightInfo {
    fn dispatch_aggregation() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Ismp::Nonce` (r:1 w:1)
    /// Proof: `Ismp::Nonce` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Timestamp::Now` (r:1 w:0)
    /// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: UNKNOWN KEY `0x52657175657374436f6d6d69746d656e74738d735dd361725e435d6df809b249` (r:1 w:1)
    /// Proof: UNKNOWN KEY `0x52657175657374436f6d6d69746d656e74738d735dd361725e435d6df809b249` (r:1 w:1)
    fn dispatch_aggregation() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `120`
        //  Estimated: `3593`
        // Minimum execution time: 62_584_000 picoseconds.
        Weight::from_parts(65_822_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
}