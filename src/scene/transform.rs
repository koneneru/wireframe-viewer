use crate::math::{EulerAngles, Mat4, Vec3};

/// Represents hte logical spatial transformation of a scene object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Transform {
    translation: Vec3,
    rotation: EulerAngles,
    scale: Vec3,
}

impl Default for Transform {
    fn default() -> Transform {
        Self {
            translation: Vec3::ZERO,
            rotation: EulerAngles::ZERO,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub(super) const fn new(translation: Vec3, rotation: EulerAngles, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub(super) const fn translation(&self) -> Vec3 {
        self.translation
    }

    pub(super) const fn rotation(&self) -> EulerAngles {
        self.rotation
    }

    pub(super) const fn scale(&self) -> Vec3 {
        self.scale
    }

    pub(super) fn set_translation(&mut self, position: Vec3) {
        self.translation = position;
    }

    pub(super) fn set_rotation(&mut self, rotation: EulerAngles) {
        self.rotation = rotation;
    }

    pub(super) fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    pub(super) fn translate(&mut self, offset: Vec3) {
        self.translation += offset;
    }

    pub(super) fn rotate(&mut self, angles: EulerAngles) {
        self.rotation += angles;
    }

    /// Calculates the local model matrix usibg T * R * S composition.
    pub(super) fn model_matrix(&self) -> Mat4 {
        let translation = Mat4::translation(self.translation);
        let scaling = Mat4::scaling(self.scale);

        let rotation_x = Mat4::rotation_x(self.rotation.x().as_radians());
        let rotation_y = Mat4::rotation_y(self.rotation.y().as_radians());
        let rotation_z = Mat4::rotation_z(self.rotation.z().as_radians());

        let rotation = rotation_z * rotation_y * rotation_x;

        translation * rotation * scaling
    }
}
