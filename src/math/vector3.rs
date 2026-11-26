use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
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

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.x.hypot(self.y).hypot(self.z)
    }

    pub fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub const fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub const fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Lineary interpoletes between this vector and `other`.
    ///
    /// The interpolation parameter is not clamped, so values outside the range
    /// `[0.0, 1.0]` perform extrapolation.
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }

    pub fn try_normalize(self) -> Option<Self> {
        if !self.is_finite() {
            return None;
        }

        let scale = self.abs().max_component();
        if scale == 0.0 {
            return None;
        }

        let scaled = self / scale;
        let len = scaled.length();

        Some(scaled / len)
    }

    // Helpers

    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    fn max_component(self) -> f32 {
        self.x.max(self.y).max(self.z)
    }
}

// Vector addition

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// Vector subtraction

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Scalar multiplication

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// Scalar division

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// Negation

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// Scalar multiplication from the left

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(a.dot(b), 32.0);
    }

    #[test]
    fn cross() {
        struct TestCase {
            name: &'static str,
            a: Vec3,
            b: Vec3,
            expected: Vec3,
        }

        let test_cases = [
            TestCase {
                name: "cross product of basis X and Y is Z",
                a: Vec3::X,
                b: Vec3::Y,
                expected: Vec3::Z,
            },
            TestCase {
                name: "cross product of basis Y and Z is X",
                a: Vec3::Y,
                b: Vec3::Z,
                expected: Vec3::X,
            },
            TestCase {
                name: "cross product of basis Z and X is Y",
                a: Vec3::Z,
                b: Vec3::X,
                expected: Vec3::Y,
            },
            TestCase {
                name: "cross product of parallel vectors is zero",
                a: Vec3::new(1.0, 2.0, 3.0),
                b: Vec3::new(2.0, 4.0, 6.0),
                expected: Vec3::ZERO,
            },
        ];

        for tc in test_cases {
            let result = tc.a.cross(tc.b);
            assert_eq!(
                result, tc.expected,
                "Test case '{}' failed: expected {:?}, got {:?}",
                tc.name, tc.expected, result,
            );

            assert_eq!(
                tc.a.cross(tc.b),
                -tc.b.cross(tc.a),
                "Test case '{}' failed: cross products must be anti-commutaive",
                tc.name,
            );
        }
    }

    #[test]
    fn lerp_interpolates_correctly() {
        let v0 = Vec3::new(2.0, 4.0, 6.0);
        let v1 = Vec3::new(10.0, 12.0, 14.0);

        let cases = [
            ("at v0", 0.0, Vec3::new(2.0, 4.0, 6.0)),
            ("at v1", 1.0, Vec3::new(10.0, 12.0, 14.0)),
            ("midpoint", 0.5, Vec3::new(6.0, 8.0, 10.0)),
            ("extrapolate below v0", -0.5, Vec3::new(-2.0, 0.0, 2.0)),
            ("extrapolate above v1", 1.5, Vec3::new(14.0, 16.0, 18.0)),
        ];

        for (name, t, want) in cases {
            assert_eq!(v0.lerp(v1, t), want, "Test case '{name}' failed, t = {t}");
        }
    }

    #[test]
    fn lerp_handles_identical_vectors() {
        let v = Vec3::new(5.0, -3.0, 8.0);

        assert_eq!(v.lerp(v, 0.0), v);
        assert_eq!(v.lerp(v, 0.5), v);
        assert_eq!(v.lerp(v, 1.0), v);
    }

    #[test]
    fn length() {
        let v = Vec3::new(2.0, 3.0, 6.0);

        assert_eq!(v.length(), 7.0);
        assert_eq!(v.length_squared(), 49.0);
    }

    #[test]
    fn try_normalize() {
        struct TestCase {
            name: &'static str,
            input: Vec3,
            expected: Option<Vec3>,
        }

        let test_cases = [
            TestCase {
                name: "normal vector",
                input: Vec3::new(2.0, 3.0, 6.0),
                expected: Some(Vec3::new(2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0)),
            },
            TestCase {
                name: "zero vector",
                input: Vec3::new(0.0, 0.0, 0.0),
                expected: None,
            },
            TestCase {
                name: "infinite vector",
                input: Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
                expected: None,
            },
        ];

        for tc in test_cases.iter() {
            let result = tc.input.try_normalize();

            match (result, tc.expected) {
                (Some(r), Some(e)) => {
                    assert!(
                        approx_eq(r.x(), e.x())
                            && approx_eq(r.y(), e.y())
                            && approx_eq(r.z(), e.z()),
                        "Test case '{}' failed: expected {:?}, got {:?}",
                        tc.name,
                        e,
                        r,
                    );
                }
                (None, None) => {}
                (result, expected) => panic!(
                    "Test case '{}' failed: expected {:?}, got {:?}",
                    tc.name, expected, result,
                ),
            }
        }
    }

    #[test]
    fn is_zero() {
        struct TestCase {
            name: &'static str,
            input: Vec3,
            expected: bool,
        }

        let test_cases = [
            TestCase {
                name: "zero vector",
                input: Vec3::ZERO,
                expected: true,
            },
            TestCase {
                name: "negative zero",
                input: Vec3::new(-0.0, 0.0, 0.0),
                expected: true,
            },
            TestCase {
                name: "non-zero vector",
                input: Vec3::new(1.0, 2.0, 3.0),
                expected: false,
            },
            TestCase {
                name: "non-zero X component",
                input: Vec3::new(-1.0, 0.0, 0.0),
                expected: false,
            },
            TestCase {
                name: "non-zero Y component",
                input: Vec3::new(0.0, -1.0, 0.0),
                expected: false,
            },
            TestCase {
                name: "non-zero Z component",
                input: Vec3::new(0.0, 0.0, -1.0),
                expected: false,
            },
        ];

        for tc in test_cases.iter() {
            assert_eq!(
                tc.input.is_zero(),
                tc.expected,
                "Test case '{}' failed",
                tc.name
            );
        }
    }

    #[test]
    fn is_finite() {
        struct TestCase {
            name: &'static str,
            input: Vec3,
            expected: bool,
        }

        let test_cases = [
            TestCase {
                name: "zero vector",
                input: Vec3::ZERO,
                expected: true,
            },
            TestCase {
                name: "finite vector",
                input: Vec3::new(1.0, -2.0, 3.0),
                expected: true,
            },
            TestCase {
                name: "positive infinity",
                input: Vec3::new(f32::INFINITY, 0.0, 0.0),
                expected: false,
            },
            TestCase {
                name: "negative infinity",
                input: Vec3::new(0.0, f32::NEG_INFINITY, 0.0),
                expected: false,
            },
            TestCase {
                name: "NaN",
                input: Vec3::new(1.0, 2.0, f32::NAN),
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
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn sub() {
        let a = Vec3::new(5.0, 7.0, 9.0);
        let b = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a - b, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn mul_scalar() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 2.0;

        assert_eq!(v * scalar, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(scalar * v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn div_scalar() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let scalar = 2.0;

        assert_eq!(v / scalar, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn add_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec3::new(5.0, 7.0, 9.0);
        v -= Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v, Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 2.0;

        assert_eq!(v, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn div_assign() {
        let mut v = Vec3::new(2.0, 4.0, 6.0);
        v /= 2.0;

        assert_eq!(v, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn negation() {
        let v = Vec3::new(1.0, -2.0, 3.0);

        assert_eq!(-v, Vec3::new(-1.0, 2.0, -3.0));
    }

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }
}
