use std::hash::{Hash, BuildHasher};
use std::collections::HashSet;

/// Interesection of an arbitrary number of sets.
///
/// The order do matter, since it will iterate over the first set and checking for the element to
/// be contained in the following sets. Thus, it is recommended, that you place the most "narrow"
/// sets in the start, to gain maximal performance.
#[macro_export]
macro_rules! intersection {
    ($a:expr, $b:expr) => {
        $a.intersection(&$b)
    };
    ($a:expr, $b:expr, $( $x:expr ),*) => {
        {
            $a.iter().filter(|x| $b.contains(x) $( && $x.contains(x))* )
        }
    }
}

/// Initialize a set with elements.
#[macro_export]
macro_rules! hashset {
    [$( $elem:expr ),*] => {
        {
            use std::collections::HashSet;

            let mut hs = HashSet::with_hasher(Default::default());

            $(
                hs.insert($elem);
            )*

            hs
        }
    };
}

/// Extension to the set API.
pub trait SetExt<T, S> {
    // TODO find a better name
    /// Create a new set.
    fn new_() -> Self;
}

impl<T, S> SetExt<T, S> for HashSet<T, S> where T: Eq + Hash, S: BuildHasher + Default {
    fn new_() -> HashSet<T, S> {
        HashSet::with_hasher(Default::default())
    }
}
