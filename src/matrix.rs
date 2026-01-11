use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Matrix4x4 {
    pub m: [[f64; 4]; 4],
}

impl Matrix4x4 {
    pub fn from_array(m: [[f64; 4]; 4]) -> Self {
        Self { m }
    }

    pub fn identity() -> Self {
        let mut m = [[0.0; 4]; 4];
        m[0][0] = 1.0;
        m[1][1] = 1.0;
        m[2][2] = 1.0;
        m[3][3] = 1.0;

        Self { m }
    }

    pub fn inverse(&self) -> Self {
        // aug = [A | I]
        let mut aug = [[0.0; 8]; 4];
        for i in 0..4 {
            for j in 0..4 {
                aug[i][j] = self.m[i][j];
            }
            aug[i][i + 4] = 1.0;
        }

        // Elimination
        for i in 0..4 {
            let pivot = aug[i][i];
            if pivot.abs() < 1e-12 {
                panic!("Tried to invert singular matrix");
            }

            for j in 0..8 {
                aug[i][j] /= pivot;
            }

            for r in 0..4 {
                if r != i {
                    let factor = aug[r][i];
                    for c in 0..8 {
                        aug[r][c] -= factor * aug[i][c];
                    }
                }
            }
        }

        let mut inv = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                inv[i][j] = aug[i][j + 4];
            }
        }

        Self { m: inv }
    }

    pub fn is_close(&self, other: &Matrix4x4) -> bool {
        let tolerance = 1e-6;

        for i in 0..4 {
            for j in 0..4 {
                if (self.m[i][j] - other.m[i][j]).abs() > tolerance {
                    return false;
                }
            }
        }

        true
    }
}

impl Mul<[f64; 4]> for Matrix4x4 {
    type Output = [f64; 4];

    fn mul(self, rhs: [f64; 4]) -> [f64; 4] {
        [
            (self.m[0][0] * rhs[0])
                + (self.m[0][1] * rhs[1])
                + (self.m[0][2] * rhs[2])
                + (self.m[0][3] * rhs[3]),
            (self.m[1][0] * rhs[0])
                + (self.m[1][1] * rhs[1])
                + (self.m[1][2] * rhs[2])
                + (self.m[1][3] * rhs[3]),
            (self.m[2][0] * rhs[0])
                + (self.m[2][1] * rhs[1])
                + (self.m[2][2] * rhs[2])
                + (self.m[2][3] * rhs[3]),
            (self.m[3][0] * rhs[0])
                + (self.m[3][1] * rhs[1])
                + (self.m[3][2] * rhs[2])
                + (self.m[3][3] * rhs[3]),
        ]
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        let mut m = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                m[i][j] = (self.m[i][0] * rhs.m[0][j])
                    + (self.m[i][1] * rhs.m[1][j])
                    + (self.m[i][2] * rhs.m[2][j])
                    + (self.m[i][3] * rhs.m[3][j]);
            }
        }

        Matrix4x4 { m }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let m = Matrix4x4::from_array([
            [1.0, 2.0, 3.0, 4.0],
            [0.0, 1.0, 4.0, 5.0],
            [2.0, 0.0, 1.0, 3.0],
            [1.0, 0.0, 0.0, 1.0],
        ]);

        let m_inv = m.inverse();

        let id = Matrix4x4::identity();
        assert!((m * m_inv).is_close(&id));
        assert!((m_inv * m).is_close(&id));
    }
}
