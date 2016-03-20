/// A simple XOR-shift derived randomizer with a decent randomness quality.
pub struct Randomizer {
    state: u64,
}

impl Randomizer {
    /// Create a new randomizer from a seed.
    pub fn new(seed: u64) -> Randomizer {
        Randomizer {
            state: seed,
        }
    }

    /// Write a buffer into the randomizer (as entropy).
    pub fn write(&mut self, buf: &[u8]) {
        for &i in buf {
            self.write_u8(i);
        }
    }

    /// Read random bytes to a buffer.
    pub fn read(&mut self, buf: &mut [u8]) {
        for i in buf {
            *i = self.read_u8();
        }
    }

    /// Read a byte from the randomizer.
    pub fn read_u8(&mut self) -> u8 {
        self.state ^= self.state.rotate_right(4).wrapping_add(0x25A45B35C4FD3DF2);
        self.state ^= self.state >> 7;
        self.state as u8
    }

    /// Write a byte into the randomizer.
    ///
    /// This is used for collecting entropy to the randomizer.
    pub fn write_u8(&mut self, b: u8) {
        self.state ^= b as u64;
        self.read_u8();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[ignore]
    #[test]
    fn test_fairness() {
        let mut hits = [0u32; 256];
        let mut rand = Randomizer::new(0);

        for _ in 0..100000000 {
            hits[rand.read_u8() as usize] += 1;
        }

        for &i in hits.iter() {
            assert!(i < 400000, "Divergence not low enough, i = {}", i);
        }
    }

    #[ignore]
    #[test]
    fn bitstream_test() {
        let mut ones = 0u64;
        let mut rand = Randomizer::new(0);

        for _ in 0..100000000 {
            ones += rand.read_u8().count_ones() as u64;
        }

        assert!(ones <= 400000000 + 5);
    }
}
