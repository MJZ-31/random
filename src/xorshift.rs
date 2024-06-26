use crate::Source;

/// An instance of the Xorshift128+ algorithm.
///
/// ## References
///
/// 1. Sebastiano Vigna, “Further Scramblings of Marsaglia’s Xorshift
///    Generators,” CoRR, 2014.
///
/// 2. https://en.wikipedia.org/wiki/Xorshift#xorshift.2B
#[derive(Clone, Copy, Debug)]
pub struct Xorshift128Plus(u64, u64);

impl Xorshift128Plus {
    /// Create an instance of the algorithm.
    ///
    /// At least one bit of the seed should be one.
    #[inline(always)]
    pub fn new(seed: [u64; 2]) -> Xorshift128Plus {
        debug_assert!(
            seed[0] | seed[1] != 0,
            "at least one bit of the seed should be one"
        );
        Xorshift128Plus(seed[0], seed[1])
    }
}

impl Source for Xorshift128Plus {
    #[inline(always)]
    fn read_u64(&mut self) -> u64 {
        let (mut x, y) = (self.0, self.1);
        self.0 = y;
        x = x ^ (x << 23);
        x = x ^ (x >> 17);
        x = x ^ y ^ (y >> 26);
        self.1 = x;
        x.wrapping_add(y)
    }
}

#[cfg(test)]
mod tests {
    use crate::Xorshift128Plus;

    #[test]
    #[should_panic]
    fn new_zero_seed() {
        let _ = Xorshift128Plus::new([0, 0]);
    }
}
