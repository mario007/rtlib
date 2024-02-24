
/// Calculate murmurhash from byte array
/// 
/// <https://github.com/explosion/murmurhash/blob/master/murmurhash/MurmurHash2.cpp>
/// 
/// * `key`: Byte array.
/// * `seed`: Initial seed used in hash calculation.
#[inline]
pub fn murmur_hash_64_a(key: &[u8], seed: u64) -> u64 {

    let m = 0xc6a4a7935bd1e995u64;
    let r = 47i32;

    let mut h = seed ^ ((key.len() as u64).wrapping_mul(m));

    let iter_chunks = key.chunks_exact(8);
    let rest = iter_chunks.remainder();
    for chunk in iter_chunks {
        let mut k = u64::from_le_bytes(chunk.try_into().unwrap());
        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);

        h ^= k;
        h = h.wrapping_mul(m);
    }
    //let rest = iter_chunks.remainder();
    for i in (0..rest.len()).rev() {
        h ^= (rest[i] as u64) << (i * 8);
    }
    if !rest.is_empty() {
        h = h.wrapping_mul(m);
    }

    h ^= h >> r;
    h = h.wrapping_mul(m);
    h ^= h >> r;

    h
}

/// Convert arguments to byte array and call function that calculate murmurhash
/// 
/// * `arg1,...`: It takes up to four arguments. Arguments are rust primitive types
/// that are converted to bytes using `to_le_bytes` method.
#[macro_export]
macro_rules! hash {
    ($e:expr) => {
        {
            murmur_hash_64_a(&($e).to_le_bytes(), 0)
        }
    };
    ($e1:expr, $e2:expr) => {
        {
            let a1 = ($e1).to_le_bytes();
            let a2 = ($e2).to_le_bytes();
            let length = a1.len() + a2.len();
            if length > 64 {
                panic!("hash macro with 2 arguments support buffer up to 64 bytes.");
            }
            let mut buffer = [0u8; 64];
            buffer[0..a1.len()].clone_from_slice(&a1);
            buffer[a1.len()..a1.len() + a2.len()].clone_from_slice(&a2);
            murmur_hash_64_a(&buffer[0..length], 0)
        }
    };
    ($e1:expr, $e2:expr, $e3: expr) => {
        {
            let a1 = ($e1).to_le_bytes();
            let a2 = ($e2).to_le_bytes();
            let a3 = ($e3).to_le_bytes();
            let a1_a2_len = a1.len() + a2.len();
            let length = a1_a2_len + a3.len();
            if length > 64 {
                panic!("hash macro with 3 arguments support buffer up to 64 bytes.");
            }
            let mut buffer = [0u8; 64];
            buffer[0..a1.len()].clone_from_slice(&a1);
            buffer[a1.len()..a1_a2_len].clone_from_slice(&a2);
            buffer[a1_a2_len..length].clone_from_slice(&a3);
            murmur_hash_64_a(&buffer[0..length], 0)
        }
    };
    ($e1:expr, $e2:expr, $e3: expr, $e4: expr) => {
        {
            let a1 = ($e1).to_le_bytes();
            let a2 = ($e2).to_le_bytes();
            let a3 = ($e3).to_le_bytes();
            let a4 = ($e4).to_le_bytes();
            let a1_a2_len = a1.len() + a2.len();
            let a1_a2_a3_len = a1_a2_len + a3.len();
            let length = a1_a2_a3_len + a4.len();
            if length > 64 {
                panic!("hash macro with 4 arguments support buffer up to 64 bytes.");
            }
            let mut buffer = [0u8; 64];
            buffer[0..a1.len()].clone_from_slice(&a1);
            buffer[a1.len()..a1_a2_len].clone_from_slice(&a2);
            buffer[a1_a2_len..a1_a2_a3_len].clone_from_slice(&a3);
            buffer[a1_a2_a3_len..length].clone_from_slice(&a4);
            murmur_hash_64_a(&buffer[0..length], 0)

            // murmur_hash_64_a(&[($e1).to_le_bytes(), ($e2).to_le_bytes(),
            // ($e3).to_le_bytes(), ($e4).to_le_bytes()].concat(), 0)
        }
    };
}

/// Calculate 64-bit hash
/// 
/// <http://zimbry.blogspot.ch/2011/09/better-bit-mixing-improving-on.html>
///
/// * `v`: input value
#[inline]
pub fn hash64(v: u64) -> u64 {
    let mut v = v;
    v ^= v >> 31;
    v = v.wrapping_mul(0x7fb5d329728ea185);
    v ^= v >> 27;
    v = v.wrapping_mul(0x81dadef4bc2dd44d);
    v ^= v >> 33;
    v
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::time::Instant;

    #[test]
    fn murmur_hash_64_a_test() {
        println!("h1 {}", hash!(566u32));
        println!("h2 {}", hash!(0u32, 1f64));
        println!("h3 {}", hash!(0u32, 1f64, 32i32));
        println!("h4 {}", hash!(0u32, 1f32, 32i32, 33u32));
        let start = Instant::now();
        let mut seed = 45454u64;
        for _i in 0..10000 {
            seed = hash!(seed, seed, seed);
        }
        let elapsed = start.elapsed();
        println!("Elapsed: {} {:.2?}", seed, elapsed);
        eprintln!("elapsed {:?}", elapsed);

        let v = 44u64;
        println!("hash value is {}", hash64(v));
    }
}
