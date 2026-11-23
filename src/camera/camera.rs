use crate::{
    camera::{ProjectionError, projection::Projection},
    math::{Angle, Mat4, Vec3, Vec4},
};

const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.001;

const MIN_DISTANCE: f32 = 0.01;
const MAX_DISTANCE: f32 = 1_000_000.0;

const WORLD_UP: Vec3 = Vec3::Y;

/// Errors returned when creating or updating a camera.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraError {
    InvalidTarget,
    InvalidDistance,
    InvalidOrientation,
}

/// Represents a virtual camera in 3D world space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    target: Vec3,
    distance: f32,
    yaw: Angle,
    pitch: Angle,
    projection: Projection,
}

impl Camera {
    pub fn new(
        target: Vec3,
        distance: f32,
        yaw: Angle,
        pitch: Angle,
        projection: Projection,
    ) -> Result<Self, CameraError> {
        if !target.is_finite() {
            return Err(CameraError::InvalidTarget);
        }

        if !distance.is_finite() || distance <= 0.0 {
            return Err(CameraError::InvalidDistance);
        }

        let camera = Self {
            target,
            distance: distance.clamp(MIN_DISTANCE, MAX_DISTANCE),
            yaw,
            pitch: Self::clamp_pitch(pitch),
            projection,
        };

        let fwd = (camera.target - camera.position())
            .try_normalize()
            .ok_or(CameraError::InvalidOrientation)?;

        if fwd.cross(WORLD_UP).try_normalize().is_none() {
            return Err(CameraError::InvalidOrientation);
        }

        Ok(camera)
    }

    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn yaw(&self) -> Angle {
        self.yaw
    }

    pub fn pitch(&self) -> Angle {
        self.pitch
    }

    pub fn set_projection(&mut self, projection: Projection) {
        self.projection = projection;
    }

    pub fn position(&self) -> Vec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();

        let offset = Vec3::new(
            self.distance * cos_pitch * sin_yaw,
            self.distance * sin_pitch,
            self.distance * cos_pitch * cos_yaw,
        );

        self.target + offset
    }

    pub fn orbit(&mut self, delta_yaw: Angle, delta_pitch: Angle) {
        self.yaw += delta_yaw;
        self.pitch = Self::clamp_pitch(self.pitch + delta_pitch);
    }

    pub fn pan(&mut self, delta_right: f32, delta_up: f32) {
        if !delta_right.is_finite() || !delta_up.is_finite() {
            return;
        }

        let (right, camera_up, _) = self.camera_axes();

        let offset = right * delta_right + camera_up * delta_up;

        self.target += offset;
    }

    pub fn zoom(&mut self, factor: f32) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }

        self.distance = (self.distance / factor).clamp(MIN_DISTANCE, MAX_DISTANCE);
    }

    pub fn view_matrix(&self) -> Mat4 {
        let (right, camera_up, forward) = self.camera_axes();

        let position = self.position();

        Mat4::from_columns([
            Vec4::new(right.x(), camera_up.x(), -forward.x(), 0.0),
            Vec4::new(right.y(), camera_up.y(), -forward.y(), 0.0),
            Vec4::new(right.z(), camera_up.z(), -forward.z(), 0.0),
            Vec4::new(
                -right.dot(position),
                -camera_up.dot(position),
                forward.dot(position),
                1.0,
            ),
        ])
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.projection.matrix()
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    fn clamp_pitch(pitch: Angle) -> Angle {
        Angle::from_radians(pitch.as_radians().clamp(-PITCH_LIMIT, PITCH_LIMIT))
    }

    fn camera_axes(&self) -> (Vec3, Vec3, Vec3) {
        let forward = (self.target - self.position())
            .try_normalize()
            .expect("Camera forward vector must be valid");

        let right = forward
            .cross(WORLD_UP)
            .try_normalize()
            .expect("Camera right vector must be valid");

        let camera_up = right.cross(forward);

        (right, camera_up, forward)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, PI};

    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn position_from_spherical_coordinates() {
        let cases = [
            (0.0, 0.0, Vec3::new(0.0, 0.0, 10.0)),
            (FRAC_PI_2, 0.0, Vec3::new(10.0, 0.0, 0.0)),
            (PI, 0.0, Vec3::new(0.0, 0.0, -10.0)),
            (
                0.0,
                FRAC_PI_4,
                Vec3::new(0.0, 10.0 * FRAC_1_SQRT_2, 10.0 * FRAC_1_SQRT_2),
            ),
        ];

        for (yaw, pitch, want) in cases {
            let cam = create_camera(10.0, yaw, pitch);

            assert_vec3_approx_eq(cam.position(), want);
        }
    }

    #[test]
    fn pitch_is_clamped() {
        let cases = [
            (
                "clamps positive value above upper pitch limit",
                2.0 * FRAC_PI_3,
                PITCH_LIMIT,
            ),
            (
                "clamps negative value below lower pitch limit",
                -2.0 * FRAC_PI_3,
                -PITCH_LIMIT,
            ),
            ("keeps value pitch limits unchanged", FRAC_PI_6, FRAC_PI_6),
        ];

        for (name, input, want) in cases {
            let mut cam = create_camera(10.0, 0.0, 0.0);

            cam.orbit(Angle::ZERO, Angle::from_radians(input));

            let got = cam.pitch().as_radians();
            assert!(
                approx_eq(got, want),
                "Test case '{name} failed: got {got}, want {want}"
            );
        }
    }

    #[test]
    fn zoom_respects_distance_limits() {
        let cases = [
            ("zoom in within limits", 10.0, 2.0, 5.0),
            ("zoom out within limits", 10.0, 0.5, 20.0),
            (
                "zoom in clamped to MIN_DISTANCE",
                10.0,
                1_000_000.0,
                MIN_DISTANCE,
            ),
            (
                "zoom out clamped to MAX_DISTANCE",
                10.0,
                0.000_001,
                MAX_DISTANCE,
            ),
        ];

        for (name, initial, factor, want) in cases {
            let mut cam = create_camera(initial, 0.0, 0.0);

            cam.zoom(factor);

            let got = cam.distance();

            assert!(
                approx_eq(got, want),
                "Test case '{name}' failed: distance got {got}, want {want}"
            );
        }
    }

    #[test]
    fn orbit_preserves_target_and_distance() {
        let mut cam = create_camera(15.0, 0.0, 0.0);

        let initial_target = cam.target();
        let initial_distance = cam.distance();

        cam.orbit(Angle::from_degrees(45.0), Angle::from_degrees(20.0));

        let current_target = cam.target();
        let current_dsitance = cam.distance();

        assert_eq!(
            current_target, initial_target,
            "target changed: got {current_target:?}, want {initial_target:?}"
        );
        assert_eq!(
            current_dsitance, initial_distance,
            "distance changed: got {current_dsitance:?}, want {initial_distance:?}"
        );

        let position = cam.position();
        let want_position = Vec3::new(0.0, 0.0, 15.0);

        assert_ne!(
            position, want_position,
            "position mismatch: got {position:?}, want {want_position:?}"
        );
    }

    #[test]
    fn pan_moves_target_and_camera_together() {
        let mut cam = create_camera(10.0, 0.0, 0.0);

        let old_target = cam.target();
        let old_position = cam.position();

        cam.pan(3.0, 2.0);

        let new_target = cam.target();
        let new_position = cam.position();

        let target_offset = new_target - old_target;
        let position_offset = new_position - old_position;

        assert_vec3_approx_eq(target_offset, position_offset);

        let distance = (cam.position() - cam.target()).length();
        assert!(
            approx_eq(distance, 10.0),
            "pan changed camera-to-target distance: got {distance}, want 10.0",
        );
    }

    #[test]
    fn view_matrix_is_correct() {
        let cam = create_camera(5.0, 0.0, 0.0);

        let got = cam.view_matrix();

        let want = Mat4::from_columns([
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, -5.0, 1.0),
        ]);

        assert_mat4_approx_eq(got, want);
    }

    #[test]
    fn view_projection_matrix_is_correct() {
        let projection =
            Projection::perspective(Angle::from_radians(FRAC_PI_2), 1.0, 0.1, 100.0).unwrap();

        let cam = Camera::new(
            Vec3::ZERO,
            5.0,
            Angle::from_radians(0.0),
            Angle::from_radians(0.0),
            projection,
        )
        .unwrap();

        let got = cam.view_projection_matrix();

        let want = Mat4::from_columns([
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, -1.002002, -1.0),
            Vec4::new(0.0, 0.0, 4.80981, 5.0),
        ]);

        assert_mat4_approx_eq(got, want);
    }

    fn assert_vec3_approx_eq(got: Vec3, want: Vec3) {
        assert!(
            approx_eq(got.x(), want.x())
                && approx_eq(got.y(), want.y())
                && approx_eq(got.z(), want.z()),
            "Vec3 assertion failed with epsilon ({EPSILON})\n  got: {got:?}\n want: {want:?}",
        )
    }

    fn assert_mat4_approx_eq(got: Mat4, want: Mat4) {
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

    fn create_camera(distance: f32, yaw: f32, pitch: f32) -> Camera {
        Camera::new(
            Vec3::ZERO,
            distance,
            Angle::from_radians(yaw),
            Angle::from_radians(pitch),
            default_projection(),
        )
        .unwrap()
    }

    fn default_projection() -> Projection {
        Projection::perspective(
            Angle::from_radians(2.0 * FRAC_PI_3),
            16.0 / 9.0,
            0.1,
            1000.0,
        )
        .unwrap()
    }
}
