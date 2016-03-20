use std::hash::Hasher;

/// A DJB2 hasher.
///
/// This performs _a lot_ better than the default SipHasher.
pub struct Djb2 {
    state: u64,
}

impl Default for Djb2 {
    fn default() -> Djb2 {
        Djb2 {
            state: 5381,
        }
    }
}

impl Hasher for Djb2 {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.state = (self.state << 5).wrapping_add(self.state).wrapping_add(b as u64);
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::hash::Hasher;

    #[test]
    fn test_correctness() {
        let mut hasher = Djb2::default();
        hasher.write(b"Hello, I'm Ticki.");
        assert_eq!(hasher.finish(), 17086230315052214372);
    }
}
