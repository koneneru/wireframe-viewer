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
