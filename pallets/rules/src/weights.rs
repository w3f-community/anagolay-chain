// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2022 Anagolay Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for an_rules
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-02-21, STEPS: `[50, ]`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/anagolay
// benchmark
// --chain=dev
// --steps=50
// --repeat=100
// --pallet=an_rules
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/rules/src/weights.rs
// --template=./templates/module-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
  traits::Get,
  weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for an_rules.
pub trait WeightInfo {
  fn create_rule() -> Weight;
}

/// Weights for an_rules using the Substrate node and recommended hardware.
pub struct AnagolayWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AnagolayWeight<T> {
  fn create_rule() -> Weight {
    (32_014_000 as Weight)
      .saturating_add(T::DbWeight::get().reads(2 as Weight))
      .saturating_add(T::DbWeight::get().writes(2 as Weight))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  fn create_rule() -> Weight {
    (32_014_000 as Weight)
      .saturating_add(RocksDbWeight::get().reads(2 as Weight))
      .saturating_add(RocksDbWeight::get().writes(2 as Weight))
  }
}
