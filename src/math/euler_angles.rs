use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use crate::math::Angle;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct EulerAngles {
    x: Angle,
    y: Angle,
    z: Angle,
}

impl EulerAngles {
    pub const ZERO: Self = EulerAngles::new(Angle::ZERO, Angle::ZERO, Angle::ZERO);

    pub const fn new(x: Angle, y: Angle, z: Angle) -> Self {
        Self { x, y, z }
    }

    pub const fn x(self) -> Angle {
        self.x
    }

    pub const fn y(self) -> Angle {
        self.y
    }

    pub const fn z(self) -> Angle {
        self.z
    }
}

impl Add for EulerAngles {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for EulerAngles {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for EulerAngles {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for EulerAngles {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for EulerAngles {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}
