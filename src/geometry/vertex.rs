use crate::math::{Vec2, Vec3};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

impl Vertex {
    pub const fn new(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }

    pub const fn position(&self) -> Vec3 {
        self.position
    }

    pub const fn normal(&self) -> Vec3 {
        self.normal
    }

    pub const fn uv(&self) -> Vec2 {
        self.uv
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn set_uv(&mut self, uv: Vec2) {
        self.uv = uv;
    }
}
