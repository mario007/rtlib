#![allow(dead_code)]


/// A trait providing methods for generating random numbers
pub trait Rng {
    /// Generate u32 random number in range [0-4294967295]
    fn rand_u32(&mut self) -> u32;

    /// Generate f32 random number in range [0-1)
    fn rand_f32(&mut self) -> f32 {
        let val = f32::from_bits(0x33800000); // 0x1p-24f, 2^-24
        (self.rand_u32() >> 8) as f32 * val
    }

    /// Generate u32 random number in range [0-range]
    fn rand_range(&mut self, range: u32) -> u32 {
        let x = self.rand_u32();
        let m = (x as u64) * (range as u64);
        (m >> 32) as u32
    }
}

/// PCG is a family of simple fast space-efficient statistically good 
/// algorithms for random number generation.
pub struct PCGRng {
    state: u64,
    inc: u64,
}

impl PCGRng {
    pub fn new(state: u64, inc: u64) -> PCGRng {
        PCGRng { state, inc }
    }
}

impl Rng for PCGRng {
    fn rand_u32(&mut self) -> u32 {
        let oldstate = self.state;
        // Advance internal state
        self.state = oldstate
            .wrapping_mul(6364136223846793005u64)
            .wrapping_add(self.inc | 1);
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << ((-(rot as i32) as u32) & 31))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn pcg_test() {
        let mut rng = PCGRng::new(0xf123456789012345, 0);
        for _i in 0..20 {
            println!("rnd: {}", rng.rand_f32());
        }
    }

    #[test]
    fn pcg_hitogram_test() {
        let mut rng = PCGRng::new(0xf12456955, 0x454555);
        let mut nums = HashMap::new();
        for _i in 0..10000 {
            let num = rng.rand_range(5);
            match nums.get(&num) {
                Some(val) => {
                    nums.insert(num, val + 1)
                },
                None => nums.insert(num, 1)
            };
        }
        print!("{:?}\n", nums.get(&0));
        print!("{:?}\n", nums.get(&1));
        print!("{:?}\n", nums.get(&2));
        print!("{:?}\n", nums.get(&3));
        print!("{:?}\n", nums.get(&4));
        print!("{:?}\n", nums.get(&5));
    }
}
