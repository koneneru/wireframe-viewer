use core::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const X: Self = Self { x: 1.0, y: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0 };

    pub const fn x(self) -> f32 {
        self.x
    }

    pub const fn y(self) -> f32 {
        self.y
    }

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn is_zero(self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();

        if len == 0.0 || !len.is_finite() {
            None
        } else {
            Some(self / len)
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        assert_eq!(a + b, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn sub() {
        let a = Vec2::new(5.0, 7.0);
        let b = Vec2::new(2.0, 3.0);

        assert_eq!(a - b, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn mul_scalar() {
        let v = Vec2::new(2.0, 3.0);
        let scalar = 2.0;

        assert_eq!(v * scalar, Vec2::new(4.0, 6.0));
        assert_eq!(scalar * v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn div_scalar() {
        let v = Vec2::new(4.0, 6.0);
        let scalar = 2.0;

        assert_eq!(v / scalar, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn dot() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        assert_eq!(a.dot(b), 11.0);
    }

    #[test]
    fn length() {
        let v = Vec2::new(3.0, 4.0);

        assert_eq!(v.length(), 5.0);
        assert_eq!(v.length_squared(), 25.0);
    }

    #[test]
    fn try_normalize() {
        struct TestCase {
            name: &'static str,
            input: Vec2,
            expected: Option<Vec2>,
        }

        let test_cases = [
            TestCase {
                name: "normal vector",
                input: Vec2::new(3.0, 4.0),
                expected: Some(Vec2::new(0.6, 0.8)),
            },
            TestCase {
                name: "zero vector",
                input: Vec2::ZERO,
                expected: None,
            },
            TestCase {
                name: "infinite vector",
                input: Vec2::new(f32::INFINITY, f32::INFINITY),
                expected: None,
            },
        ];

        for tc in test_cases.iter() {
            let result = tc.input.try_normalize();

            match (result, tc.expected) {
                (Some(r), Some(e)) => {
                    assert!(
                        approx_eq(r.x, e.x) && approx_eq(r.y, e.y),
                        "Test case '{}' failed: expected {:?}, got {:?}",
                        tc.name,
                        e,
                        r
                    );
                }
                (None, None) => {}
                (result, expected) => panic!(
                    "Test case '{}' failed: expected {:?}, got {:?}",
                    tc.name, expected, result
                ),
            }
        }
    }

    #[test]
    fn add_assign() {
        let mut v = Vec2::new(1.0, 2.0);
        v += Vec2::new(3.0, 4.0);

        assert_eq!(v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn sub_assign() {
        let mut v = Vec2::new(5.0, 7.0);
        v -= Vec2::new(2.0, 3.0);

        assert_eq!(v, Vec2::new(3.0, 4.0));
    }
    #[test]
    fn mul_assign() {
        let mut v = Vec2::new(2.0, 3.0);
        v *= 2.0;

        assert_eq!(v, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn div_assign() {
        let mut v = Vec2::new(4.0, 6.0);
        v /= 2.0;

        assert_eq!(v, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn negation() {
        let v = Vec2::new(1.0, -2.0);

        assert_eq!(-v, Vec2::new(-1.0, 2.0));
    }

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }
}
