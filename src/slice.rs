extern crate collections;

/// Types which can be converted to an `Option<T>`.
pub trait AsOption<T> {
    /// Convert this value into an `Option`.
    fn as_option(&self) -> Option<T>;
}

impl AsOption<usize> for usize {
    fn as_option(&self) -> Option<usize> {
        Some(*self)
    }
}

impl AsOption<usize> for Option<usize> {
    fn as_option(&self) -> Option<usize> {
        *self
    }
}

use self::collections::range::RangeArgument;
use std::ops::Range;
use std::cmp;

/// Bounded slice abstraction.
///
/// # Code Migration
///
/// `foo[a..b]` => `foo.get_slice(a..b)`
///
/// `foo[a..]` => `foo.get_slice(a..)`
///
/// `foo[..b]` => `foo.get_slice(..b)`
///
pub trait GetSlice {
    /// Panic safely get a slice of a value. When out of bound, the bound will be saturated.
    fn get_slice<T: Into<Option<usize>> + Copy, U: RangeArgument<T>>(&self, a: U) -> &Self;
    /// Panic safely get a mutable slice of a value. When out of bound, the bound will be saturated.
    fn get_slice_mut<T: Into<Option<usize>> + Copy, U: RangeArgument<T>>(&mut self, a: U) -> &mut Self;
}

fn bound<T: Into<Option<usize>> + Copy, U: RangeArgument<T>>(len: usize, a: U) -> Range<usize> {
    let start = cmp::min(a.start().and_then(|x| (*x).into()).unwrap_or(0), len);
    let end = cmp::min(a.end().and_then(|x| (*x).into()).unwrap_or(len), len);

    if start <= end {
        start..end
    } else {
        0..0
    }
}

impl GetSlice for str {
    fn get_slice<T: Into<Option<usize>> + Copy, U: RangeArgument<T>>(&self, a: U) -> &Self {
        &self[bound(self.len(), a)]
    }

    fn get_slice_mut<T: Into<Option<usize>> + Copy, U: RangeArgument<T>>(&mut self, a: U) -> &mut Self {
        let len = self.len();
        &mut self[bound(len, a)]
    }
}

impl<T> GetSlice for [T] {
    fn get_slice<U: Into<Option<usize>> + Copy, V: RangeArgument<U>>(&self, a: V) -> &Self {
        &self[bound(self.len(), a)]
    }

    fn get_slice_mut<U: Into<Option<usize>> + Copy, V: RangeArgument<U>>(&mut self, a: V) -> &mut Self {
        let len = self.len();
        &mut self[bound(len, a)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_slice() {
        let arr = [1, 2, 3];
        assert_eq!(arr.get_slice(1..42), &[2, 3]);
        assert_eq!(arr.get_slice(20..42), &[]);
        assert_eq!(arr.get_slice(0..), &arr);
        assert_eq!(arr.get_slice::<usize, _>(..), &arr);
        assert_eq!(arr.get_slice(..42), &arr);
        assert_eq!(arr.get_slice(Some(1)..None), &[2, 3]);
        assert_eq!(arr.get_slice(Some(1)..Some(2)), &[2]);
        let mut vec = Vec::new();
        assert!(vec.get_slice(Some(1)..Some(2)).is_empty());
        vec.push(3);
        assert_eq!(vec.get_slice(0..2), &[3]);
        assert!(vec.get_slice(1..2).is_empty());
    }
}
