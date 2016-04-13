/// A LGC based, non-cryptographic, pseudo-random number generator with full cycle length (2^64 - 1).
///
/// To avoid hyperplanes, we apply a bijective function on the output.
pub struct Randomizer {
    state: u64,
}

impl Randomizer {
    /// Create a new randomizer from a seed.
    pub fn new(seed: u64) -> Randomizer {
        Randomizer {
            state: seed.wrapping_add(0xDEADBEEFDEADBEEF),
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
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state.wrapping_mul(1152921504735157271).rotate_right(2) ^ 0xFAB00105C0DE) as u8
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
    fn test_residue() {
        fn fairness(n: u64) {
            const TESTS: u64 = 10000000;
            use std::mem;

            let mut hits = vec![0; n as usize];
            let mut rand = Randomizer::new(0);

            for _ in 0..TESTS {
                let mut buf = [0; 8];
                rand.read(&mut buf);

                unsafe {
                    hits[(mem::transmute::<[u8; 8], u64>(buf) % n) as usize] += 1;
                }
            }

            for &i in hits.iter() {
                assert!(i < TESTS / n, "Residue divergence not low enough, n = {}, div = {}", n, i as f64 / TESTS as f64 * n as f64);
            }
        }

        for i in 3..1000 {
            fairness(i);
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

        assert!(ones <= 400000000, "Bits are biased. {}% ones.", ones as f64 / 400000000.0);
    }
}
