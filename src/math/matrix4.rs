use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

use super::{Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    columns: [Vec4; 4],
}

impl Mat4 {
    pub const ZERO: Self = Self {
        columns: [Vec4::ZERO; 4],
    };

    pub const IDENTITY: Self = Self {
        columns: [Vec4::X, Vec4::Y, Vec4::Z, Vec4::W],
    };

    pub const fn from_columns(columns: [Vec4; 4]) -> Self {
        Self { columns }
    }

    pub const fn translation(offset: Vec3) -> Self {
        Self::from_columns([
            Vec4::X,
            Vec4::Y,
            Vec4::Z,
            Vec4::new(offset.x(), offset.y(), offset.z(), 1.0),
        ])
    }

    pub const fn scaling(scale: Vec3) -> Self {
        Self::from_columns([
            Vec4::new(scale.x(), 0.0, 0.0, 0.0),
            Vec4::new(0.0, scale.y(), 0.0, 0.0),
            Vec4::new(0.0, 0.0, scale.z(), 0.0),
            Vec4::W,
        ])
    }

    pub fn rotation_x(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::from_columns([
            Vec4::X,
            Vec4::new(0.0, cos, sin, 0.0),
            Vec4::new(0.0, -sin, cos, 0.0),
            Vec4::W,
        ])
    }

    pub fn rotation_y(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::from_columns([
            Vec4::new(cos, 0.0, -sin, 0.0),
            Vec4::Y,
            Vec4::new(sin, 0.0, cos, 0.0),
            Vec4::W,
        ])
    }

    pub fn rotation_z(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();

        Self::from_columns([
            Vec4::new(cos, sin, 0.0, 0.0),
            Vec4::new(-sin, cos, 0.0, 0.0),
            Vec4::Z,
            Vec4::W,
        ])
    }

    pub const fn column(&self, index: usize) -> Vec4 {
        self.columns[index]
    }

    pub fn row(&self, index: usize) -> Vec4 {
        Vec4::new(
            self.columns[0][index],
            self.columns[1][index],
            self.columns[2][index],
            self.columns[3][index],
        )
    }

    pub fn is_finite(&self) -> bool {
        self.columns.iter().all(|column| column.is_finite())
    }

    pub fn transpose(self) -> Self {
        Self::from_columns([self.row(0), self.row(1), self.row(2), self.row(3)])
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Mat4 {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.columns.iter_mut().zip(rhs.columns) {
            *lhs += rhs;
        }
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign for Mat4 {
    fn sub_assign(&mut self, rhs: Self) {
        for (lhs, rhs) in self.columns.iter_mut().zip(rhs.columns) {
            *lhs -= rhs;
        }
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_columns(self.columns.map(|column| column * rhs))
    }
}

impl MulAssign<f32> for Mat4 {
    fn mul_assign(&mut self, rhs: f32) {
        for column in &mut self.columns {
            *column *= rhs;
        }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.columns[0] * rhs.x()
            + self.columns[1] * rhs.y()
            + self.columns[2] * rhs.z()
            + self.columns[3] * rhs.w()
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_columns([
            self * rhs.columns[0],
            self * rhs.columns[1],
            self * rhs.columns[2],
            self * rhs.columns[3],
        ])
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Mat4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::from_columns(self.columns.map(|column| column / rhs))
    }
}

impl DivAssign<f32> for Mat4 {
    fn div_assign(&mut self, rhs: f32) {
        for column in &mut self.columns {
            *column /= rhs;
        }
    }
}

impl Neg for Mat4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_columns(self.columns.map(|column| -column))
    }
}

impl Index<(usize, usize)> for Mat4 {
    type Output = f32;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.columns[column][row]
    }
}

#[cfg(test)]
mod tests {
    use super::{Mat4, Vec3, Vec4};

    const EPSILON: f32 = 1e-6;

    // --- Consts & ctors ---

    #[test]
    fn zero_matrix() {
        let mat = Mat4::ZERO;

        for row in 0..4 {
            for col in 0..4 {
                assert_eq!(mat[(row, col)], 0.0);
            }
        }
    }

    #[test]
    fn identity_matrix() {
        let mat = Mat4::IDENTITY;

        for row in 0..4 {
            for col in 0..4 {
                let expected = if row == col { 1.0 } else { 0.0 };

                assert_eq!(mat[(row, col)], expected);
            }
        }
    }

    #[test]
    fn from_columns() {
        let columns = [
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        ];

        let mat = Mat4::from_columns(columns);

        for col in 0..4 {
            for row in 0..4 {
                let got = mat[(row, col)];
                let excepted = columns[col][row];

                assert_eq!(
                    got, excepted,
                    "Incorrect element at position [{row}][{col}]: got {got:?}, excepted {excepted:?}"
                );
            }
        }
    }

    #[test]
    fn column() {
        let columns = [
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        ];

        let mat = Mat4::from_columns(columns);

        for i in 0..4 {
            let got = mat.column(i);
            let excepted = columns[i];

            assert_eq!(
                got, excepted,
                "Incorrect column at index {i}: got {got:?}, excepted {excepted:?}"
            );
        }
    }

    #[test]
    fn row() {
        let columns = [
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        ];

        let mat = Mat4::from_columns(columns);

        for i in 0..4 {
            let got = mat.row(i);
            let expected = Vec4::new(columns[0][i], columns[1][i], columns[2][i], columns[3][i]);

            assert_eq!(
                got, expected,
                "Incorrect row at index {i}: got {got:?}, expected {expected:?}",
            );
        }
    }

    // --- Arithmetics ---

    #[test]
    fn add_matrices() {
        assert_mat4_approx_eq(matrix_a() + matrix_b(), matrix_sum_a_b());
    }

    #[test]
    fn add_assign_matrices() {
        let mut matrix = matrix_a();

        matrix += matrix_b();

        assert_mat4_approx_eq(matrix, matrix_sum_a_b());
    }

    #[test]
    fn subtract_matrices() {
        assert_mat4_approx_eq(matrix_sum_a_b() - matrix_a(), matrix_b());
    }

    #[test]
    fn sub_assign_matrices() {
        let mut matrix = matrix_sum_a_b();

        matrix -= matrix_a();

        assert_mat4_approx_eq(matrix, matrix_b());
    }

    #[test]
    fn multiply_matrix_by_scalar() {
        assert_eq!(matrix_a() * 2.0, matrix_a_doubled());
    }

    #[test]
    fn multiply_scalar_by_matrix() {
        assert_eq!(2.0 * matrix_a(), matrix_a_doubled())
    }

    #[test]
    fn multiply_assign_matrix_by_scalar() {
        let mut matrix = matrix_a();

        matrix *= 2.0;

        assert_eq!(matrix, matrix_a_doubled())
    }

    #[test]
    fn divide_by_scalar() {
        assert_eq!(matrix_a_doubled() / 2.0, matrix_a())
    }

    #[test]
    fn divide_assign_by_scalar() {
        let mut matrix = matrix_a_doubled();

        matrix /= 2.0;

        assert_eq!(matrix, matrix_a());
    }

    #[test]
    fn negate_matrix() {
        let matrix = matrix_a();
        let expected = Mat4::from_columns([
            Vec4::new(-1.0, -2.0, -3.0, -4.0),
            Vec4::new(-5.0, -6.0, -7.0, -8.0),
            Vec4::new(-9.0, -10.0, -11.0, -12.0),
            Vec4::new(-13.0, -14.0, -15.0, -16.0),
        ]);

        assert_eq!(-matrix, expected);
    }

    // --- Matrix Multiplication ---

    #[test]
    fn multiply_matrix_by_vector() {
        let matrix = matrix_a();
        let vector = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let result = matrix * vector;

        assert_vec4_approx_eq(result, Vec4::new(90.0, 100.0, 110.0, 120.0));
    }

    #[test]
    fn multiply_matrices() {
        let left = matrix_a();

        let right = Mat4::from_columns([
            Vec4::new(2.0, 0.0, 1.0, 0.0),
            Vec4::new(1.0, 3.0, 0.0, 1.0),
            Vec4::new(0.0, 2.0, 1.0, 0.0),
            Vec4::new(1.0, 0.0, 2.0, 1.0),
        ]);

        let expected = Mat4::from_columns([
            Vec4::new(11.0, 14.0, 17.0, 20.0),
            Vec4::new(29.0, 34.0, 39.0, 44.0),
            Vec4::new(19.0, 22.0, 25.0, 28.0),
            Vec4::new(32.0, 36.0, 40.0, 44.0),
        ]);

        assert_eq!(left * right, expected);
    }

    #[test]
    fn identity_is_left_neutral_element() {
        let matrix = matrix_a();

        assert_eq!(matrix * Mat4::IDENTITY, matrix);
    }

    #[test]
    fn identity_is_right_neutral_element() {
        let matrix = matrix_a();

        assert_eq!(Mat4::IDENTITY * matrix, matrix);
    }

    #[test]
    fn matrix_multiplication_is_not_commutative() {
        assert_ne!(matrix_a() * matrix_b(), matrix_b() * matrix_a());
    }

    #[test]
    fn multiply_assign_matrices() {
        let mut matrix = Mat4::IDENTITY;

        matrix *= matrix_a();

        assert_eq!(matrix, matrix_a());
    }

    // --- Transpose ---

    #[test]
    fn transpose_matrix() {
        let matrix = matrix_a();
        let transposed = matrix.transpose();

        for col in 0..4 {
            for row in 0..4 {
                assert_eq!(matrix[(row, col)], transposed[(col, row)]);
            }
        }
    }

    #[test]
    fn transpose_twice_returns_original_matrix() {
        let matrix = matrix_a();

        assert_eq!(matrix.transpose().transpose(), matrix);
    }

    #[test]
    fn transpose_identity_returns_identity() {
        assert_eq!(Mat4::IDENTITY.transpose(), Mat4::IDENTITY);
    }

    // --- Finiteness ---

    #[test]
    fn is_finite_returns_true_for_finite_matrix() {
        assert!(matrix_a().is_finite());
    }

    #[test]
    fn is_finite_returns_fasle_for_matrix_with_infinity() {
        let matrix = Mat4::from_columns([
            Vec4::new(f32::INFINITY, 0.0, 0.0, 0.0),
            Vec4::ZERO,
            Vec4::ZERO,
            Vec4::ZERO,
        ]);

        assert!(!matrix.is_finite());
    }

    #[test]
    fn is_finite_returns_fasle_for_matrix_with_nan() {
        let matrix = Mat4::from_columns([
            Vec4::new(f32::NAN, 0.0, 0.0, 0.0),
            Vec4::ZERO,
            Vec4::ZERO,
            Vec4::ZERO,
        ]);

        assert!(!matrix.is_finite());
    }

    // --- Transformation ---

    #[test]
    fn translation() {
        let got = Mat4::translation(Vec3::new(2.0, 3.0, 4.0));

        let expected =
            Mat4::from_columns([Vec4::X, Vec4::Y, Vec4::Z, Vec4::new(2.0, 3.0, 4.0, 1.0)]);

        assert_mat4_approx_eq(got, expected);
    }

    #[test]
    fn scaling() {
        let got = Mat4::scaling(Vec3::new(2.0, 3.0, 4.0));

        let expected = Mat4::from_columns([
            Vec4::new(2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 4.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ]);

        assert_mat4_approx_eq(got, expected);
    }

    #[test]
    fn rotation_x_transforms_basis_y_to_z() {
        let rotation = Mat4::rotation_x(std::f32::consts::FRAC_PI_2);

        assert_vec4_approx_eq(rotation * Vec4::Y, Vec4::Z);
    }

    #[test]
    fn rotation_y_transforms_basis_z_to_x() {
        let rotation = Mat4::rotation_y(std::f32::consts::FRAC_PI_2);

        assert_vec4_approx_eq(rotation * Vec4::Z, Vec4::X);
    }

    #[test]
    fn rotation_z_transforms_basis_x_to_y() {
        let rotation = Mat4::rotation_z(std::f32::consts::FRAC_PI_2);

        assert_vec4_approx_eq(rotation * Vec4::X, Vec4::Y);
    }

    // --- Helpers ---

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() <= EPSILON
    }

    fn assert_vec4_approx_eq(got: Vec4, expected: Vec4) {
        const COMPONENTS: [&str; 4] = ["x", "y", "z", "w"];

        for (i, name) in COMPONENTS.iter().enumerate() {
            assert!(
                approx_eq(got[i], expected[i]),
                "Vector mismatch at {name}: got {}, expected {}",
                got[i],
                expected[i],
            );
        }
    }

    fn assert_mat4_approx_eq(got: Mat4, expected: Mat4) {
        for col in 0..4 {
            for row in 0..4 {
                let got = got[(row, col)];
                let expected = expected[(row, col)];

                assert!(
                    approx_eq(got, expected),
                    "Incorrect result matrix at position [{row}][{col}]: got {got:?}, excepted {expected:?}"
                );
            }
        }
    }

    fn matrix_a() -> Mat4 {
        Mat4::from_columns([
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        ])
    }

    fn matrix_b() -> Mat4 {
        Mat4::from_columns([
            Vec4::new(17.0, 18.0, 19.0, 20.0),
            Vec4::new(21.0, 22.0, 23.0, 24.0),
            Vec4::new(25.0, 26.0, 27.0, 28.0),
            Vec4::new(29.0, 30.0, 31.0, 32.0),
        ])
    }

    fn matrix_sum_a_b() -> Mat4 {
        Mat4::from_columns([
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
            Vec4::new(34.0, 36.0, 38.0, 40.0),
            Vec4::new(42.0, 44.0, 46.0, 48.0),
        ])
    }

    fn matrix_a_doubled() -> Mat4 {
        Mat4::from_columns([
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        ])
    }
}
