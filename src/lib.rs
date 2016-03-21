#![feature(collections)]
#![feature(collections_range)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(stmt_expr_attributes)]
#![feature(unicode)]

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
pub mod io;
/// Non-cryptographic pseudo-random number generators.
pub mod rand;
/// Panic-safe slicing.
pub mod slice;
