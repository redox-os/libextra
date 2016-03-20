use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

/// Extension for maps.
pub trait MapExt<K, V, S> {
    /// Get the value if it exists, if not set it and return a reference.
    fn get_or(&mut self, key: K, or: V) -> &V;
    /// Get the value if it exists, if not set it and return a mutable reference.
    fn get_or_mut(&mut self, key: K, or: V) -> &mut V;
    // TODO find a better name
    /// Create a new map.
    fn new_() -> Self;
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

    fn new_() -> HashMap<K, V, S> {
        HashMap::with_hasher(Default::default())
    }
}
