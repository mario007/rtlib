
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

/// Inner product in twice the working precision
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn inner_product_test() {
        let x = &[1.0, 2.0, 3.0];
        let y = &[1.0, 2.0, 3.0];
        let p = inner_product(x, y);
        assert_eq!(p, 14.0);
    }
}
