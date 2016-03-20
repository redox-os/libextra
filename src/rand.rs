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

    /// Read a number from the randomizer.
    pub fn read_rand(&mut self) -> u8 {
        self.state ^= self.state.rotate_right(4).wrapping_add(0x25A45B35C4FD3DF2);
        self.state ^= self.state >> 7;
        self.state as u8
    }

    /// Write a number into the randomizer.
    ///
    /// This is used for collecting entropy to the randomizer.
    pub fn write_rand(&mut self, b: u8) {
        self.state ^= b as u64;
        self.read_rand();
    }
}
