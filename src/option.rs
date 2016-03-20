use unreachable::unreachable;
use io::WriteExt;

use std::process;
use std::error::Error;
use std::io::{self, Write};

/// Extension for Option-like types
pub trait OptionalExt {
    /// The "success" variant of this optional type.
    type Succ;

    /// Unwrap or abort program with exit code
    fn try(self, stderr: &mut io::Stderr) -> Self::Succ;

    /// Unwrap or abort the program with failed exit code and custom error message
    fn fail<'a>(self, err: &'a str, stderr: &mut io::Stderr) -> Self::Succ;

    /// An unwrapping where the fail-case is not checked and threaten as statical unreachable.
    unsafe fn unchecked_unwrap(self) -> Self::Succ;
}

impl<T, U: Error> OptionalExt for Result<T, U> {
    type Succ = T;

    fn try(self, stderr: &mut io::Stderr) -> T {
        let mut stderr = stderr.lock();

        match self {
            Ok(succ) => succ,
            Err(e) => {
                // We ignore the results to avoid stack overflow (because of unbounded
                // recursion).
                let _ = stderr.write(b"error: ");
                let _ = stderr.write(e.description().as_bytes());
                let _ = stderr.write(b"\n");
                let _ = stderr.flush();
                process::exit(1);
            },
        }
    }

    fn fail<'a>(self, err: &'a str, stderr: &mut io::Stderr) -> T {
        let mut stderr = stderr.lock();

        match self {
            Ok(succ) => succ,
            Err(_) => {
                let _ = stderr.write(b"error: ");
                let _ = stderr.write(err.as_bytes());
                let _ = stderr.write(b"\n");
                let _ = stderr.flush();
                process::exit(1);
            },
        }
    }

    unsafe fn unchecked_unwrap(self) -> T {
        if let Ok(x) = self {
            x
        } else {
            unreachable()
        }
    }
}

impl<T> OptionalExt for Option<T> {
    type Succ = T;

    fn try(self, stderr: &mut io::Stderr) -> T {
        let mut stderr = stderr.lock();

        match self {
            Some(succ) => succ,
            None => {
                let _ = stderr.writeln(b"error: (no message)\n");
                let _ = stderr.flush();
                process::exit(1);
            },
        }
    }

    fn fail<'a>(self, err: &'a str, stderr: &mut io::Stderr) -> T {
        let mut stderr = stderr.lock();

        match self {
            Some(succ) => succ,
            None => {
                let _ = stderr.write(b"error:");
                let _ = stderr.write(err.as_bytes());
                let _ = stderr.write(b"\n");
                let _ = stderr.flush();
                process::exit(1);
            },
        }
    }

    unsafe fn unchecked_unwrap(self) -> T {
        if let Some(x) = self {
            x
        } else {
            unreachable()
        }
    }
}

/// Like `try!`, but accepting `Option`s instead.
#[macro_export]
macro_rules! try_some {
    ($x:expr) => {
        if let Some(x) = $x {
            x
        } else {
            return None;
        }
    };
    ($x:expr => $y:expr) => {
        if let Some(x) = $x {
            x
        } else {
            return $y;
        }
    };
}

/// A generalization of `try!()`.
///
/// If this optional (`Option`-like) type is successful, return the inner value. If not, evaluate
/// the expression right to the arrow.
///
/// ## How is this different than `unwrap_or()`?
///
/// Unwrap or evaluates inside an closure, thus it cannot access various statement related to
/// control flow. For example, `return` will return the value from the closure, whereas `maybe!`,
/// will expand the statement inline, such that the current function will return.
#[macro_export]
macro_rules! maybe {
    ($a:expr => $b:expr) => {
        if let Some(x) = $a.into_iter().next() {
            x
        } else {
            $b
        }
    };
}
