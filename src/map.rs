use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Extension for maps.
pub trait MapExt<K, V, S> {
    /// Get the value if it exists, if not set it and return a reference.
    fn get_or(&mut self, key: K, or: V) -> &V;
    /// Get the value if it exists, if not set it and return a mutable reference.
    fn get_or_mut(&mut self, key: K, or: V) -> &mut V;
    /// Create a new map, with default hasher.
    fn new_default() -> Self;
}

impl<K, V, S> MapExt<K, V, S> for HashMap<K, V, S>
    where K: Eq + Hash + Clone,
          S: BuildHasher + Default {
    fn get_or(&mut self, key: K, or: V) -> &V {
        if self.contains_key(&key) {
            self.get(&key).unwrap()
        } else {
            self.insert(key.clone(), or);
            self.get(&key).unwrap()
        }
    }

    fn get_or_mut(&mut self, key: K, or: V) -> &mut V {
        if self.contains_key(&key) {
            self.get_mut(&key).unwrap()
        } else {
            self.insert(key.clone(), or);
            self.get_mut(&key).unwrap()
        }
    }

    fn new_default() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::hash::BuildHasherDefault;
    use hash::Djb2;

    #[test]
    fn test_get_or() {
        let mut map = HashMap::new();

        map.get_or(5, "dQw4w9WgXcQ");
        assert_eq!(*map.get(&5).unwrap(), "dQw4w9WgXcQ"); // Rick Roll'd.
        map.get_or(6, "This is the Redox book, which will go through (almost) everything about Redox");
        assert_eq!(*map.get(&6).unwrap(), "This is the Redox book, which will go through (almost) everything about Redox");
    }

    #[test]
    fn test_new_default() {
        let _: HashMap<u64, u64, BuildHasherDefault<Djb2>> = HashMap::new_default();
    }
}
