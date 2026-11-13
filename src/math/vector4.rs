use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub const fn x(self) -> f32 {
        self.x
    }

    pub const fn y(self) -> f32 {
        self.y
    }

    pub const fn z(self) -> f32 {
        self.z
    }

    pub const fn w(self) -> f32 {
        self.w
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub(crate) fn component(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Vec4 component index out of bounds: {index}"),
        }
    }
}

// Vector addition

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

// Vector subtraction

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

// Scalar multiplication

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

// Scalar division

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

// Negation

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

// Scalar multiplication from the left

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants() {
        assert_eq!(Vec4::ZERO, Vec4::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::ONE, Vec4::new(1.0, 1.0, 1.0, 1.0));

        assert_eq!(Vec4::X, Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(Vec4::Y, Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(Vec4::Z, Vec4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(Vec4::W, Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn new() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
        assert_eq!(v.w(), 4.0);
    }

    #[test]
    fn dot() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        assert_eq!(a.dot(b), 70.0);
    }

    #[test]
    fn is_finite() {
        struct TestCase {
            name: &'static str,
            input: Vec4,
            expected: bool,
        }

        let test_cases = [
            TestCase {
                name: "zero vector",
                input: Vec4::ZERO,
                expected: true,
            },
            TestCase {
                name: "finite vector",
                input: Vec4::new(1.0, 2.0, 3.0, 4.0),
                expected: true,
            },
            TestCase {
                name: "positive infinity",
                input: Vec4::new(f32::INFINITY, 2.0, 3.0, 4.0),
                expected: false,
            },
            TestCase {
                name: "negative infinity",
                input: Vec4::new(1.0, f32::NEG_INFINITY, 3.0, 4.0),
                expected: false,
            },
            TestCase {
                name: "NaN",
                input: Vec4::new(1.0, 2.0, f32::NAN, 4.0),
                expected: false,
            },
            TestCase {
                name: "w component is infinite",
                input: Vec4::new(1.0, 2.0, 3.0, f32::INFINITY),
                expected: false,
            },
        ];

        for tc in test_cases {
            assert_eq!(
                tc.input.is_finite(),
                tc.expected,
                "Test case '{}' failed",
                tc.name
            );
        }
    }

    #[test]
    fn add() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(5.0, 6.0, 7.0, 8.0);

        assert_eq!(a + b, Vec4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn add_assign() {
        let mut v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        v += Vec4::new(5.0, 6.0, 7.0, 8.0);

        assert_eq!(v, Vec4::new(6.0, 8.0, 10.0, 12.0));
    }

    #[test]
    fn sub() {
        let a: Vec4 = Vec4::new(5.0, 7.0, 9.0, 11.0);
        let b: Vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(a - b, Vec4::new(4.0, 5.0, 6.0, 7.0));
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec4::new(5.0, 7.0, 9.0, 11.0);
        v -= Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v, Vec4::new(4.0, 5.0, 6.0, 7.0));
    }

    #[test]
    fn mul_scalar() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let scalar = 2.0;

        assert_eq!(v * scalar, Vec4::new(2.0, 4.0, 6.0, 8.0));
        assert_eq!(scalar * v, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        v *= 2.0;

        assert_eq!(v, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }

    #[test]
    fn div_scalar() {
        let v = Vec4::new(2.0, 4.0, 6.0, 8.0);
        let scalar = 2.0;

        assert_eq!(v / scalar, Vec4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn div_assign() {
        let mut v = Vec4::new(2.0, 4.0, 6.0, 8.0);
        v /= 2.0;

        assert_eq!(v, Vec4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn neg() {
        let v = Vec4::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-v, Vec4::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn mul() {
        let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let scalar = 2.0;

        assert_eq!(v * scalar, Vec4::new(2.0, 4.0, 6.0, 8.0));
    }
}
