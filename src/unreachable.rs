/// An unreachable function.
///
/// In debug mode this is equivalent to calling `unreachable!()`. In release mode, however, this
/// will assume the call to be _statically unreachable_. Revoking this in reachable code is
/// undefined behavior, and might result in various unsafe conditions. Hence, the `unsafe`.
pub unsafe fn unreachable() -> ! {
    #[cfg(debug)]
    unreachable!();

    #[cfg(not(debug))]
    std::hint::unreachable_unchecked();
}

