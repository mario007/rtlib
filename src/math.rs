
/// difference_of_products computes a * b - c * d in a way that avoids catastrophic cancellation.
#[inline(always)]
pub fn difference_of_products(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let cd = c * d;
    let err = (-c).mul_add(d, cd);
    let dop = a.mul_add(b, -cd);
    dop + err
}

#[inline]
fn two_sum(a: f32, b: f32) -> (f32, f32) {
    let x = a + b;
    let z = x - a;
    let y = (a - (x - z)) + (b - z);
    (x, y)
}

#[inline]
fn two_product_fma(a: f32, b: f32) -> (f32, f32) {
    let x = a * b;
    let y = a.mul_add(b, -x);
    (x, y)
}

/// calculate inner product in twice the working precision
pub fn inner_product(x: &[f32], y: &[f32]) -> f32 {
    let (mut p, mut s) = two_product_fma(x[0], y[0]);
    for i in 1..x.len() {
        let (h, r) = two_product_fma(x[i], y[i]);
        let (p1, q) = two_sum(p, h);
        p = p1;
        s += q + r;
    }
    p + s
}


/// Return premutated index
/// 
/// * `index`: number to permutate.
/// * `n`: Total numbers used in permutation.
/// * `seed`: used for permutation.
#[inline]
pub fn permutation_element(index: u32, n: u32, seed: u32) -> u32 {
    let mut i = index;
    let mut w = n - 1;
    w |= w >> 1;
    w |= w >> 2;
    w |= w >> 4;
    w |= w >> 8;
    w |= w >> 16;

    loop {
        i ^= seed;
        i = i.wrapping_mul(0xe170893d);
        i ^= seed >> 16;
        i ^= (i & w) >> 4;
        i ^= seed >> 8;
        i = i.wrapping_mul(0x0929eb3f);
        i ^= seed >> 23;
        i ^= (i & w) >> 1;
        i *= 1 | seed >> 27;
        i = i.wrapping_mul(0x6935fa69);
        i ^= (i & w) >> 11;
        i = i.wrapping_mul(0x74dcb303);
        i ^= (i & w) >> 2;
        i = i.wrapping_mul(0x9e501cc3);
        i ^= (i & w) >> 2;
        i = i.wrapping_mul(0xc860a3df);
        i &= w;
        i ^= i >> 5;

        if i < n { break; }
    }

    return (i.wrapping_add(seed)) % n;
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn inner_product_test() {
        let x = &[1.0, 2.0, 3.0];
        let y = &[1.0, 2.0, 3.0];
        let p = inner_product(x, y);
        assert_eq!(p, 14.0);
    }

    #[test]
    fn permutation_test() {
        let seed: u32 = 255552;
        let total: u32 = 259;
        let mut ids = HashSet::new();
        for i in 0..total {
            ids.insert(permutation_element(i, total, seed));
        }
        for i in 0..total {
            assert_eq!(true, ids.contains(&i));
        }
    }
}
