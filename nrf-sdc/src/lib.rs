//! Nordic SoftDevice Controller and Multiprotocol Service Layer binaries.
//!
//! This crate carries no Rust API: linking it makes the MPSL and SoftDevice
//! Controller symbols available, and exports the nrfxlib headers so that a
//! dependent can generate bindings against them.

#![no_std]

/// Directory holding the nrfxlib headers, plus the `stub/` shims that let them
/// parse without a copy of the nRF MDK.
pub const INCLUDE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/include");

