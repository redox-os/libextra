//! This crate contains various useful stuff, that libstd lacks of.

#![warn(missing_docs)]

/// Extension of functionality for `Option`-like types.
#[macro_use]
pub mod option;
/// Mathematical vectors.
pub mod vec;
/// Unreachability markers.
pub mod unreachable;
/// Extension for `HashMap`-like structures.
#[cfg(not(target_os = "redox"))]
pub mod map;
/// Extension for `HashSet`-like structures.
#[cfg(not(target_os = "redox"))]
#[macro_use]
pub mod set;
/// Primitives for debugging.
#[macro_use]
pub mod debug;
/// Faster hashing than the default.
pub mod hash;
/// Extras for IO.
#[macro_use]
pub mod io;
/// Non-cryptographic pseudo-random number generators.
pub mod rand;
