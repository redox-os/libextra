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
    fn new_default() -> Self;
}

impl<T, S> SetExt<T, S> for HashSet<T, S> where T: Eq + Hash, S: BuildHasher + Default {
    fn new_default() -> HashSet<T, S> {
        HashSet::with_hasher(Default::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use std::hash::BuildHasherDefault;
    use hash::Djb2;

    #[test]
    fn test_intersection() {
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let mut set3 = HashSet::new();
        set1.insert(4);
        set1.insert(5);
        set2.insert(4);
        set2.insert(7);
        set3.insert(4);
        set3.insert(16);

        let mut int = intersection!(set1, set2, set3);
        assert_eq!(int.next(), Some(&4));
        assert!(int.next().is_none());
    }

    #[test]
    fn test_initializer() {
        let set: HashSet<_, BuildHasherDefault<Djb2>> = hashset!(2, 3, 5, 7, 11, 13);

        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(set.contains(&5));
        assert!(set.contains(&7));
        assert!(set.contains(&11));
        assert!(set.contains(&13));

        assert!(set.len() == 6);
    }

    #[test]
    fn test_new_default() {
        let _: HashSet<u64, BuildHasherDefault<Djb2>> = HashSet::new_default();
    }
}
