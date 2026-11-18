use std::rc::Rc;

use super::transform::Transform;
use crate::geometry::Mesh;
use crate::math::{EulerAngles, Mat4, Vec3};

/// Represnts an object placed in athe virtual scene.
#[derive(Debug, Clone, PartialEq)]
pub struct SceneObject {
    mesh: Rc<Mesh>,
    transform: Transform,
    visible: bool,
}

impl SceneObject {
    pub fn new(mesh: Rc<Mesh>) -> Self {
        Self {
            mesh,
            transform: Transform::default(),
            visible: true,
        }
    }

    pub fn mesh(&self) -> &Mesh {
        self.mesh.as_ref()
    }

    pub fn mesh_handle(&self) -> Rc<Mesh> {
        Rc::clone(&self.mesh)
    }

    pub const fn is_visible(&self) -> bool {
        self.visible
    }

    pub const fn position(&self) -> Vec3 {
        self.transform.translation()
    }

    pub const fn rotation(&self) -> EulerAngles {
        self.transform.rotation()
    }

    pub const fn scale(&self) -> Vec3 {
        self.transform.scale()
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.transform.set_translation(position);
    }

    pub fn set_rotation(&mut self, rotation: EulerAngles) {
        self.transform.set_rotation(rotation);
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.transform.set_scale(scale);
    }

    pub fn move_by(&mut self, offset: Vec3) {
        self.transform.translate(offset);
    }

    pub fn rotate(&mut self, angles: EulerAngles) {
        self.transform.rotate(angles);
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn switch_visibility(&mut self) {
        self.visible = !self.visible;
    }

    /// Returns the object's model matrix.
    pub fn model_matrix(&self) -> Mat4 {
        self.transform.model_matrix()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    mod construction {
        use super::*;

        #[test]
        fn new_object_has_defaulf_transform_and_visibility() {
            let obj = create_test_object();

            assert_eq!(obj.position(), Vec3::ZERO);
            assert_eq!(obj.rotation(), EulerAngles::ZERO);
            assert_eq!(obj.scale(), Vec3::ONE);
            assert!(obj.is_visible());
        }

        #[test]
        fn new_object_owns_provided_mesh() {
            let mesh = create_test_mesh();
            let obj = SceneObject::new(Rc::clone(&mesh));

            assert!(Rc::ptr_eq(&mesh, &obj.mesh_handle()));
        }

        #[test]
        fn mesh_handle_shares_ownership() {
            let mesh = create_test_mesh();
            let obj = SceneObject::new(Rc::clone(&mesh));

            let handle = obj.mesh_handle();

            assert!(Rc::ptr_eq(&mesh, &handle));
            assert_eq!(Rc::strong_count(&mesh), 3);
        }
    }

    mod transform {
        use super::*;

        use crate::math::Angle;

        #[test]
        fn set_position_updates_object_position() {
            let mut obj = create_test_object();

            let position = Vec3::new(10.0, 20.0, 30.0);

            obj.set_position(position);

            assert_eq!(obj.position(), position);
        }

        #[test]
        fn move_by_updates_object_position() {
            let mut obj = create_test_object();

            obj.set_position(Vec3::new(1.0, 2.0, 3.0));
            obj.move_by(Vec3::new(4.0, 5.0, 6.0));

            assert_eq!(obj.position(), Vec3::new(5.0, 7.0, 9.0));
        }

        #[test]
        fn set_rotation_updates_object_rotation() {
            let mut obj = create_test_object();

            let rotation = EulerAngles::new(
                Angle::from_radians(0.1),
                Angle::from_radians(0.2),
                Angle::from_radians(0.3),
            );

            obj.set_rotation(rotation);

            assert_eq!(obj.rotation(), rotation);
        }

        #[test]
        fn rotate_adds_angles_to_current_ratation() {
            let mut obj = create_test_object();

            let initial = EulerAngles::new(
                Angle::from_radians(0.1),
                Angle::from_radians(0.2),
                Angle::from_radians(0.3),
            );

            let offset = EulerAngles::new(
                Angle::from_radians(0.4),
                Angle::from_radians(0.5),
                Angle::from_radians(0.6),
            );

            let want = EulerAngles::new(
                Angle::from_radians(0.5),
                Angle::from_radians(0.7),
                Angle::from_radians(0.9),
            );

            obj.set_rotation(initial);
            obj.rotate(offset);

            assert_eq!(obj.rotation(), want);
        }

        #[test]
        fn set_scale_updates_object_scale() {
            let mut obj = create_test_object();

            let scale = Vec3::new(2.0, 3.0, 4.0);

            obj.set_scale(scale);

            assert_eq!(obj.scale(), scale);
        }
    }

    mod visibility {
        use super::*;

        #[test]
        fn hide_makes_object_invisible() {
            let mut obj = create_test_object();

            obj.hide();

            assert!(!obj.is_visible());
        }

        #[test]
        fn show_makes_object_visible() {
            let mut obj = create_test_object();

            obj.hide();
            obj.show();

            assert!(obj.is_visible());
        }

        #[test]
        fn switch_visibility_toggles_visibility() {
            let mut obj = create_test_object();

            assert!(obj.is_visible());

            obj.switch_visibility();
            assert!(!obj.is_visible());

            obj.switch_visibility();
            assert!(obj.is_visible());
        }
    }

    mod model_matrix {
        use crate::math::{Angle, Vec4};

        use super::*;

        #[test]
        fn identity_for_default_transform() {
            let obj = create_test_object();

            assert_matrix_approx_eq(obj.model_matrix(), Mat4::IDENTITY);
        }

        #[test]
        fn uses_correct_transform_composition_order() {
            let mut obj = create_test_object();

            obj.set_position(Vec3::new(5.0, 6.0, 7.0));
            obj.set_rotation(EulerAngles::new(
                Angle::from_degrees(90.0),
                Angle::from_degrees(90.0),
                Angle::from_degrees(90.0),
            ));
            obj.set_scale(Vec3::new(2.0, 3.0, 4.0));

            let got = obj.model_matrix();

            let want = Mat4::from_columns([
                Vec4::new(0.0, 0.0, -2.0, 0.0),
                Vec4::new(0.0, 3.0, 0.0, 0.0),
                Vec4::new(4.0, 0.0, 0.0, 0.0),
                Vec4::new(5.0, 6.0, 7.0, 1.0),
            ]);

            assert_matrix_approx_eq(got, want);
        }
    }

    fn create_test_object() -> SceneObject {
        SceneObject::new(create_test_mesh())
    }

    fn create_test_mesh() -> Rc<Mesh> {
        Rc::new(Mesh::new(Vec::new(), Vec::new()).expect("Failed to create test mesh"))
    }

    fn assert_matrix_approx_eq(got: Mat4, want: Mat4) {
        for row in 0..4 {
            for col in 0..4 {
                let got = got[(row, col)];
                let want = want[(row, col)];
                assert!(
                    approx_eq(got, want),
                    "Matrix mismatch at ({row}, {col}): got {got}, want {want}",
                );
            }
        }
    }

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }
}
