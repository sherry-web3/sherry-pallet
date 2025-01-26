//! # Cherry Count Pallet
//!
//! A pallet that demonstrates simple storage and extrinsic functions in Substrate.
//! This pallet maintains a single storage item `CherryCount` and provides extrinsics to add to its value
//! and retrieve the current count.
//! 
//! 
//! 1. Pallet Config
//! 2. Pallet Storage
//! 3. Pallet Event
//! 4. Pallet Error
//! 5. Pallet Calls

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;


#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;


    // Creating pallet structure
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    

    #[pallet::config]
    pub trait Config: frame_system::Config {
    }

    
}