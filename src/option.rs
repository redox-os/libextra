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

    /// Consume an optional type, and write a warning to stderr if it is the "fail" value. If it is
    /// the "success" value, return that. If not, return None.
    fn warn(self, stderr: &mut io::Stderr) -> Option<Self::Succ>;

    /// An unwrapping where the fail-case is not checked and threaten as statical unreachable.
    unsafe fn unchecked_unwrap(self) -> Self::Succ;
}

impl<T, U: Error> OptionalExt for Result<T, U> {
    type Succ = T;

    fn try(self, stderr: &mut io::Stderr) -> T {
        self.unwrap_or_else(|e| {
            let mut stderr = stderr.lock();

            // We ignore the results to avoid stack overflow (because of unbounded
            // recursion).
            let _ = stderr.write(b"error: ");
            let _ = stderr.write(e.description().as_bytes());
            let _ = stderr.write(b"\n");
            let _ = stderr.flush();
            process::exit(1);
        })
    }

    fn fail<'a>(self, err: &'a str, stderr: &mut io::Stderr) -> T {
        self.unwrap_or_else(|_| {
            let mut stderr = stderr.lock();

            let _ = stderr.write(b"error: ");
            let _ = stderr.write(err.as_bytes());
            let _ = stderr.write(b"\n");
            let _ = stderr.flush();
            process::exit(1);
        })
    }

    fn warn(self, stderr: &mut io::Stderr) -> Option<T> {
        if let Err(ref e) = self {
            let mut stderr = stderr.lock();

            let _ = stderr.write(b"warning: ");
            let _ = stderr.write(e.description().as_bytes());
            let _ = stderr.write(b"\n");
            let _ = stderr.flush();
        }

        self.ok()
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
        self.unwrap_or_else(|| {
            let mut stderr = stderr.lock();

            let _ = stderr.writeln(b"error: (no message)\n");
            let _ = stderr.flush();
            process::exit(1);
        })
    }

    fn fail<'a>(self, err: &'a str, stderr: &mut io::Stderr) -> T {
        self.unwrap_or_else(|| {
            let mut stderr = stderr.lock();

            let _ = stderr.write(b"error:");
            let _ = stderr.write(err.as_bytes());
            let _ = stderr.write(b"\n");
            let _ = stderr.flush();
            process::exit(1);
        })
    }

    fn warn(self, stderr: &mut io::Stderr) -> Option<T> {
        if self.is_none() {
            let mut stderr = stderr.lock();

            let _ = stderr.writeln(b"warning: (no message)\n");
            let _ = stderr.flush();
        }

        self
    }

    unsafe fn unchecked_unwrap(self) -> T {
        if let Some(x) = self {
            x
        } else {
            unreachable()
        }
    }
}

/// Extension for `Option<T>`.
pub trait OptionExt: OptionalExt {
    /// Filter this Option.
    ///
    /// This takes a closure which returns a boolean. If true, nothing will change. If false, it
    /// will set the option to `None`.
    fn filter<F>(self, filter: F) -> Self where F: FnOnce(&Self::Succ) -> bool;
}

impl<T> OptionExt for Option<T> {
    fn filter<F>(self, filter: F) -> Self where F: FnOnce(&T) -> bool {
        match self {
            Some(ref x) if !filter(x) => None,
            _ => self,
        }
    }
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
    ($x:expr) => {
        if let Some(x) = $x {
            x
        } else {
            return None;
        }
    };
    ($a:expr => $b:expr) => {
        if let Some(x) = $a.into_iter().next() {
            x
        } else {
            $b
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_maybe() {
        fn func() -> Option<u8> {
            loop {
                maybe!(None => break);
            }

            maybe!(None);
            unreachable!();
        }

        assert!(func().is_none());
    }

    #[test]
    fn test_filter() {
        assert_eq!(Some(3).filter(|x| x & 1 == 0), None);
        assert_eq!(Some(2).filter(|x| x & 1 == 0), Some(2));
    }
}
