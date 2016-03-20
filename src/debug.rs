/// Write to console if debug_logging is activated (with newline).
#[macro_export]
macro_rules! debugln {
    ($fmt:expr) => (debug!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (debug!(concat!($fmt, "\n"), $($arg)*));
}

/// Write to console if debug_logging is activated (without newline).
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug_logging")]
        print!($($arg)*);
    };
}
