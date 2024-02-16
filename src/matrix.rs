use std::ops::{Add, Mul};

use crate::math::{inner_product, difference_of_products};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4x4 {
    m: [[f32; 4]; 4]
}

impl Matrix4x4 {
    pub fn new(m: [[f32; 4]; 4]) -> Matrix4x4 {
        Matrix4x4 {m}
    }

    pub fn identity() -> Matrix4x4 {
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Matrix4x4::new(m)
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let m = [
            [self.m[0][0],   self.m[1][0], self.m[2][0], self.m[3][0]],
            [self.m[0][1],   self.m[1][1], self.m[2][1], self.m[3][1]],
            [self.m[0][2],   self.m[1][2], self.m[2][2], self.m[3][2]],
            [self.m[0][3],   self.m[1][3], self.m[2][3], self.m[3][3]],
        ];
        Matrix4x4::new(m)
    }

    pub fn determinant(&self) -> f32 {
        let m = self.m;
        let s0 = difference_of_products(m[0][0], m[1][1], m[1][0], m[0][1]);
        let s1 = difference_of_products(m[0][0], m[1][2], m[1][0], m[0][2]);
        let s2 = difference_of_products(m[0][0], m[1][3], m[1][0], m[0][3]);

        let s3 = difference_of_products(m[0][1], m[1][2], m[1][1], m[0][2]);
        let s4 = difference_of_products(m[0][1], m[1][3], m[1][1], m[0][3]);
        let s5 = difference_of_products(m[0][2], m[1][3], m[1][2], m[0][3]);

        let c0 = difference_of_products(m[2][0], m[3][1], m[3][0], m[2][1]);
        let c1 = difference_of_products(m[2][0], m[3][2], m[3][0], m[2][2]);
        let c2 = difference_of_products(m[2][0], m[3][3], m[3][0], m[2][3]);

        let c3 = difference_of_products(m[2][1], m[3][2], m[3][1], m[2][2]);
        let c4 = difference_of_products(m[2][1], m[3][3], m[3][1], m[2][3]);
        let c5 = difference_of_products(m[2][2], m[3][3], m[3][2], m[2][3]);

        difference_of_products(s0, c5, s1, c4) + 
        difference_of_products(s2, c3, -s3, c2) +
        difference_of_products(s5, c0, s4, c1)
    }

    /// Calculate inverse of matrix
    /// 
    /// <http://www.geometrictools.com/Documentation/LaplaceExpansionTheorem.pdf>
    pub fn inverse(&self) -> Option<Matrix4x4> {
        let m = self.m;
        let s0 = difference_of_products(m[0][0], m[1][1], m[1][0], m[0][1]);
        let s1 = difference_of_products(m[0][0], m[1][2], m[1][0], m[0][2]);
        let s2 = difference_of_products(m[0][0], m[1][3], m[1][0], m[0][3]);

        let s3 = difference_of_products(m[0][1], m[1][2], m[1][1], m[0][2]);
        let s4 = difference_of_products(m[0][1], m[1][3], m[1][1], m[0][3]);
        let s5 = difference_of_products(m[0][2], m[1][3], m[1][2], m[0][3]);

        let c0 = difference_of_products(m[2][0], m[3][1], m[3][0], m[2][1]);
        let c1 = difference_of_products(m[2][0], m[3][2], m[3][0], m[2][2]);
        let c2 = difference_of_products(m[2][0], m[3][3], m[3][0], m[2][3]);

        let c3 = difference_of_products(m[2][1], m[3][2], m[3][1], m[2][2]);
        let c4 = difference_of_products(m[2][1], m[3][3], m[3][1], m[2][3]);
        let c5 = difference_of_products(m[2][2], m[3][3], m[3][2], m[2][3]);

        let determinant = inner_product(&[s0, -s1, s2, s3, s5, -s4], &[c5, c4, c3, c2, c0, c1]);
        if determinant == 0.0 {
            return None
        }
        let s = 1.0 / determinant;

        let mut inv = [[0.0f32; 4]; 4];
        inv[0][0] = s * inner_product(&[m[1][1], m[1][3], -m[1][2]], &[c5, c3, c4]);
        inv[0][1] = s * inner_product(&[-m[0][1], m[0][2], -m[0][3]], &[c5, c4, c3]);
        inv[0][2] = s * inner_product(&[m[3][1], m[3][3], -m[3][2]], &[s5, s3, s4]);
        inv[0][3] = s * inner_product(&[-m[2][1], m[2][2], -m[2][3]], &[s5, s4, s3]);

        inv[1][0] = s * inner_product(&[-m[1][0], m[1][2], -m[1][3]], &[c5, c2, c1]);
        inv[1][1] = s * inner_product(&[m[0][0], m[0][3], -m[0][2]], &[c5, c1, c2]);
        inv[1][2] = s * inner_product(&[-m[3][0], m[3][2], -m[3][3]], &[s5, s2, s1]);
        inv[1][3] = s * inner_product(&[m[2][0], m[2][3], -m[2][2]], &[s5, s1, s2]);

        inv[2][0] = s * inner_product(&[m[1][0], m[1][3], -m[1][1]], &[c4, c0, c2]);
        inv[2][1] = s * inner_product(&[-m[0][0], m[0][1], -m[0][3]], &[c4, c2, c0]);
        inv[2][2] = s * inner_product(&[m[3][0], m[3][3], -m[3][1]], &[s4, s0, s2]);
        inv[2][3] = s * inner_product(&[-m[2][0], m[2][1], -m[2][3]], &[s4, s2, s0]);

        inv[3][0] = s * inner_product(&[-m[1][0], m[1][1], -m[1][2]], &[c3, c1, c0]);
        inv[3][1] = s * inner_product(&[m[0][0], m[0][2], -m[0][1]], &[c3, c0, c1]);
        inv[3][2] = s * inner_product(&[-m[3][0], m[3][1], -m[3][2]], &[s3, s1, s0]);
        inv[3][3] = s * inner_product(&[m[2][0], m[2][2], -m[2][1]], &[s3, s0, s1]);
        Some(Matrix4x4::new(inv))
    }

}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, m2: Matrix4x4) -> Self::Output {
        let mut m = [[0.0f32; 4]; 4];
        for (i, row) in m.iter_mut().enumerate() {
            for (j, val) in row.iter_mut().enumerate().take(4) {
                *val = inner_product(
                    &[self.m[i][0], self.m[i][1], self.m[i][2], self.m[i][3]],
                    &[m2.m[0][j], m2.m[1][j], m2.m[2][j], m2.m[3][j]]
                );
            }
        }
        Matrix4x4::new(m)
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut m = [[0.0f32; 4]; 4];
        for (i, row) in m.iter_mut().enumerate() {
            for (j, val) in row.iter_mut().enumerate().take(4) {
                *val = self.m[i][j] * rhs;
            }
        }
        Matrix4x4::new(m)  
    }
}

impl Mul<Matrix4x4> for f32 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut m = [[0.0f32; 4]; 4];
        for (i, row) in m.iter_mut().enumerate() {
            for (j, val) in row.iter_mut().enumerate().take(4) {
                *val = rhs.m[i][j] * self;
            }
        }
        Matrix4x4::new(m)  
    }
}

impl Add for Matrix4x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut m = [[0.0f32; 4]; 4];
        for (i, row) in m.iter_mut().enumerate() {
            for (j, val) in row.iter_mut().enumerate().take(4) {
                *val = self.m[i][j] + rhs.m[i][j];
            }
        }
        Matrix4x4::new(m)   
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mul_matrix() {
        let m1 = Matrix4x4::identity();
        let m2 = Matrix4x4::identity();
        let m = m1 * m2;
        assert_eq!(m.m, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]);

        let m3 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]]);
        let m4 = Matrix4x4::new([[1.0, 1.0, 1.0, 1.0], [2.0, 2.0, 2.0, 2.0], [3.0, 3.0, 3.0, 3.0], [4.0, 4.0, 4.0, 4.0]]);
        let m = m3 * m4;
        assert_eq!(m.m, [[30.0, 30.0, 30.0, 30.0], [70.0, 70.0, 70.0, 70.0], [110.0, 110.0, 110.0, 110.0], [150.0, 150.0, 150.0, 150.0]]);

    }

    #[test]
    fn determinant() {
        let m = Matrix4x4::new([[4.0, 4.0, 1.0, 1.0], [7.0, 8.0, 8.0, 0.0], [6.0, 6.0, 7.0, 6.0], [1.0, 2.0, 4.0, 4.0]]);
        assert_eq!(m.determinant(), 166.0);
    }

    #[test]
    fn inverse_mat() {
        let m = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 6.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 0.0, 5.0, 16.0]]);
        let inv = m.inverse();
        assert_eq!(inv.unwrap().m, [[-0.4047619, -0.14285715, 0.16666667, 0.04761905], [-0.39285716, 0.71428573, -0.25, -0.071428575],
                        [0.5, -1.0, 0.5, 0.0], [0.17261904, 0.42857143, -0.2916667, 0.023809524]])
    }

    #[test]
    fn transpose_mat() {
        let m = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]]);
        let tran = m.transpose();
        assert_eq!(tran.m,[[1.0, 5.0, 9.0, 13.0], [2.0, 6.0, 10.0, 14.0], [3.0, 7.0, 11.0, 15.0], [4.0, 8.0, 12.0, 16.0]])
    }

    #[test]
    fn add_multiply_mat() {
        let m1 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]]);
        let m2 = Matrix4x4::new([[1.0, 9.0, 3.0, 4.0], [5.0, 3.0, 7.0, 8.0], [9.0, 10.0, 1.0, 12.0], [13.0, 14.0, 1.0, 16.0]]);
        let m3 = m1 * 2.0;
        let m4 = 2.0 * m1;
        let m5 = m1 + m2;
        assert_eq!(m3.m,[[2.0, 4.0, 6.0, 8.0], [10.0, 12.0, 14.0, 16.0], [18.0, 20.0, 22.0, 24.0], [26.0, 28.0, 30.0, 32.0]]);
        assert_eq!(m4.m,[[2.0, 4.0, 6.0, 8.0], [10.0, 12.0, 14.0, 16.0], [18.0, 20.0, 22.0, 24.0], [26.0, 28.0, 30.0, 32.0]]);
        assert_eq!(m5.m,[[2.0, 11.0, 6.0, 8.0], [10.0, 9.0, 14.0, 16.0], [18.0, 20.0, 12.0, 24.0], [26.0, 28.0, 16.0, 32.0]]);
    }
}
