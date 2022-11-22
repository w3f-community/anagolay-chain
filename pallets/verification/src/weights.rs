// This file is part of Anagolay Network.

// Copyright (C) 2019-2022 Anagolay Network.

//! Autogenerated weights for verification
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-10, STEPS: `50`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/anagolay
// benchmark
// pallet
// --chain
// dev
// --steps
// 50
// --repeat
// 100
// --pallet
// verification
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --output
// ./pallets/verification/src/weights.rs
// --template
// ./templates/module-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
  sp_std::marker::PhantomData,
  traits::Get,
  weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for verification.
pub trait WeightInfo {
  fn request_verification() -> Weight;
  fn perform_verification() -> Weight;
}

/// Weights for verification using the Substrate node and recommended hardware.
pub struct AnagolayWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AnagolayWeight<T> {
  // Storage: Verification VerificationRequestByAccountIdAndVerificationContext (r:1 w:1)
  // Storage: Poe ProofTotal (r:1 w:1)
  // Storage: Poe ProofByProofIdAndAccountId (r:0 w:1)
  // Storage: Poe ProofIdsByVerificationContext (r:0 w:1)
  fn request_verification() -> Weight {
    (68_260_000 as Weight)
      .saturating_add(T::DbWeight::get().reads(2 as Weight))
      .saturating_add(T::DbWeight::get().writes(4 as Weight))
  }
  // Storage: Verification VerificationRequestByAccountIdAndVerificationContext (r:1 w:1)
  fn perform_verification() -> Weight {
    (30_300_000 as Weight)
      .saturating_add(T::DbWeight::get().reads(1 as Weight))
      .saturating_add(T::DbWeight::get().writes(1 as Weight))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  // Storage: Verification VerificationRequestByAccountIdAndVerificationContext (r:1 w:1)
  // Storage: Poe ProofTotal (r:1 w:1)
  // Storage: Poe ProofByProofIdAndAccountId (r:0 w:1)
  // Storage: Poe ProofIdsByVerificationContext (r:0 w:1)
  fn request_verification() -> Weight {
    (68_260_000 as Weight)
      .saturating_add(RocksDbWeight::get().reads(2 as Weight))
      .saturating_add(RocksDbWeight::get().writes(4 as Weight))
  }
  // Storage: Verification VerificationRequestByAccountIdAndVerificationContext (r:1 w:1)
  fn perform_verification() -> Weight {
    (30_300_000 as Weight)
      .saturating_add(RocksDbWeight::get().reads(1 as Weight))
      .saturating_add(RocksDbWeight::get().writes(1 as Weight))
  }
}
