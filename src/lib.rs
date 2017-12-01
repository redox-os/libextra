//! This crate contains various useful stuff, that libstd lacks of.

#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(stmt_expr_attributes)]

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

/// Macro for failing for a specified cause.
///
/// `fail_because` is a solution to this segment of code being used
/// many many many times:
///
/// ```
/// let var = function_returning_result().unwrap_or_else(|err| {
///     eprintln!("program_name: {}", err);
///     exit(1);
/// });
/// ```
///
/// `fail_because` provides a convenient way to fail your program for any
/// reason that implements `Display`. It prints the name of the executable
/// that was invoked, the error, and then exits with the specified return
/// code (if specified).
///
/// # Examples
/// Here is the above scenario with `fail_because`
///
/// ```
/// let var = function_returning_result().unwrap_or_else(|err| fail_because!(err) );
/// ```
///
/// `fail_because` can also take an exit code:
///
/// ```
/// fail_because!(err, 8);
/// ```
#[macro_export]
macro_rules! fail_because {
    ($err:expr) => ({
        fail_because!($err, 1);
    });
    ($err:expr, $exit_code:expr) => ({
        let arg = std::env::args().next().unwrap();
        let invoc = std::path::Path::new(&arg).file_name().unwrap().to_string_lossy();
        eprintln!("{}: {}", invoc, $err);
        std::process::exit($exit_code);
    })
}
