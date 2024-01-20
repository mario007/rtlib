
/// difference_of_products computes a * b - c * d in a way that avoids catastrophic cancellation.
#[inline(always)]
pub fn difference_of_products(a: f32, b: f32, c: f32, d: f32) -> f32 {
    let cd = c * d;
    let err = (-c).mul_add(d, cd);
    let dop = a.mul_add(b, -cd);
    dop + err
}
