use crate::math::{Angle, Mat4, Vec4};

/// Errors returned when creating a projection configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectionError {
    InvalidFieldOfView,
    InvalidAspectRatio,
    InvalidClippingPlanes,
    InvalidOrthographicBounds,
    UnrepresentableMatrix,
}

/// Defines the camera projection configuration and stores its matrix.
///
/// The matrix is calculated and validated during construction.
/// Therefore, every successffully created `Projection` contains
/// a matrix with finite `f32` coefficients.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projection {
    kind: ProjectionKind,
    matrix: Mat4,
}

/// Internal representation of projection parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
enum ProjectionKind {
    Perspective {
        fov: Angle,
        aspect: f32,
        near: f32,
        far: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

impl Projection {
    /// Creates a perspective projection configuration.
    ///
    /// The field of view must be in range `(0, PI)`.
    /// The aspect ratio and near clipping plane must be positive.
    /// The far clipping plane must be greater than the near plane.
    pub fn perspective(
        fov: Angle,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Result<Self, ProjectionError> {
        let fov_rad = fov.as_radians();

        if fov_rad <= 0.0 {
            return Err(ProjectionError::InvalidFieldOfView);
        }

        if !aspect.is_finite() || aspect <= 0.0 {
            return Err(ProjectionError::InvalidAspectRatio);
        }

        if !near.is_finite() || !far.is_finite() || near <= 0.0 || far <= near {
            return Err(ProjectionError::InvalidClippingPlanes);
        }

        let matrix = Self::perspective_matrix(fov, aspect, near, far)
            .ok_or(ProjectionError::UnrepresentableMatrix)?;

        Ok(Self {
            kind: ProjectionKind::Perspective {
                fov,
                aspect,
                near,
                far,
            },
            matrix,
        })
    }

    /// Creates an orthographic projection configuration.
    ///
    /// The horizontal and vertical bounds must be finite and strictly
    /// increasing. The near clipping plane must be positive. The far clipping
    /// plane must be greater than the near plane.
    pub fn orthographic(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Result<Self, ProjectionError> {
        let bounds = [left, right, bottom, top, near, far];

        if bounds.iter().any(|val| !val.is_finite())
            || right <= left
            || top <= bottom
            || near <= 0.0
            || far <= near
        {
            return Err(ProjectionError::InvalidOrthographicBounds);
        }

        let matrix = Self::orthographic_matrix(left, right, bottom, top, near, far)
            .ok_or(ProjectionError::UnrepresentableMatrix)?;

        Ok(Self {
            kind: ProjectionKind::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            },
            matrix,
        })
    }

    /// Returns perspective parameters, if this is a perspective projection.
    pub fn perspective_parameters(self) -> Option<(Angle, f32, f32, f32)> {
        match self.kind {
            ProjectionKind::Perspective {
                fov,
                aspect,
                near,
                far,
            } => Some((fov, aspect, near, far)),
            ProjectionKind::Orthographic { .. } => None,
        }
    }

    /// Returns orthographic parameters, if this is a orthographic projection.
    pub fn orthographic_parameters(self) -> Option<(f32, f32, f32, f32, f32, f32)> {
        match self.kind {
            ProjectionKind::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => Some((left, right, bottom, top, near, far)),
            ProjectionKind::Perspective { .. } => None,
        }
    }

    /// Ruturns the precomputed projection matrix.
    pub fn matrix(self) -> Mat4 {
        self.matrix
    }

    /// Crates a right-handed perspective projection matrix.
    ///
    /// The resulting normalized device coordinate depth range is [-1, 1].
    fn perspective_matrix(fov: Angle, aspect: f32, near: f32, far: f32) -> Option<Mat4> {
        let fov = f64::from(fov.as_radians());
        let aspect = f64::from(aspect);
        let near = f64::from(near);
        let far = f64::from(far);

        let focal_length = 1.0 / (fov * 0.5).tan();
        let depth = far - near;

        let x_scale = focal_length / aspect;
        let y_scale = focal_length;
        let z_scale = -(far + near) / depth;
        let z_offset = -(2.0 * far * near) / depth;

        if ![x_scale, y_scale, z_scale, z_offset]
            .into_iter()
            .all(is_f32_representable)
        {
            return None;
        }

        Some(Mat4::from_columns([
            Vec4::new(x_scale as f32, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y_scale as f32, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z_scale as f32, -1.0),
            Vec4::new(0.0, 0.0, z_offset as f32, 0.0),
        ]))
    }

    /// Creates a right-handed orthographic projection matrix.
    ///
    /// The resulting normalized device coordinate depth range is [-1, 1].
    fn orthographic_matrix(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Option<Mat4> {
        let left = f64::from(left);
        let right = f64::from(right);
        let bottom = f64::from(bottom);
        let top = f64::from(top);
        let near = f64::from(near);
        let far = f64::from(far);

        let width = right - left;
        let height = top - bottom;
        let depth = far - near;

        let x_scale = 2.0 / width;
        let y_scale = 2.0 / height;
        let z_scale = -2.0 / depth;

        let x_offset = -(right + left) / width;
        let y_offset = -(top + bottom) / height;
        let z_offset = -(far + near) / depth;

        if ![x_scale, y_scale, z_scale, x_offset, y_offset, z_offset]
            .into_iter()
            .all(is_f32_representable)
        {
            return None;
        }

        Some(Mat4::from_columns([
            Vec4::new(x_scale as f32, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y_scale as f32, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z_scale as f32, 0.0),
            Vec4::new(x_offset as f32, y_offset as f32, z_offset as f32, 1.0),
        ]))
    }
}

fn is_f32_representable(value: f64) -> bool {
    value.is_finite() && value >= f64::from(f32::MIN) && value <= f64::from(f32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    mod orthographic {
        use super::*;

        const DEFAULT_LEFT: f32 = -1.0;
        const DEFAULT_RIGHT: f32 = 1.0;
        const DEFAULT_BOTTOM: f32 = -1.0;
        const DEFAULT_TOP: f32 = 1.0;
        const DEFAULT_NEAR: f32 = 0.1;
        const DEFAULT_FAR: f32 = 100.0;

        #[test]
        fn accepts_valid_horizontal_bounds() {
            let cases = [
                ("symetric bounds", -1.0, 1.0),
                ("positive bounds", 1.0, 10.0),
                ("negative bounds", -10.0, -1.0),
                ("close bounds", 1.0, 1.001),
                ("large bounds", -10_000.0, 10_000.0),
                ("minimum left bound", f32::MIN, 1.0),
                ("maximum right bound", -1.0, f32::MAX),
            ];

            for (name, left, right) in cases {
                let result = Projection::orthographic(
                    left,
                    right,
                    DEFAULT_BOTTOM,
                    DEFAULT_TOP,
                    DEFAULT_NEAR,
                    DEFAULT_FAR,
                );

                assert_valid(name, result);
            }
        }

        #[test]
        fn accepts_valid_vertical_bounds() {
            let cases = [
                ("symetric bounds", -1.0, 1.0),
                ("positive bounds", 1.0, 10.0),
                ("negative bounds", -10.0, -1.0),
                ("close bounds", 1.0, 1.001),
                ("large bounds", -10_000.0, 10_000.0),
                ("minimum bottom bound", f32::MIN, 1.0),
                ("maximum top bound", -1.0, f32::MAX),
            ];

            for (name, bottom, top) in cases {
                let result = Projection::orthographic(
                    DEFAULT_LEFT,
                    DEFAULT_RIGHT,
                    bottom,
                    top,
                    DEFAULT_NEAR,
                    DEFAULT_FAR,
                );

                assert_valid(name, result);
            }
        }

        #[test]
        fn accepts_valid_planes() {
            let cases = [
                ("smallest positive near", f32::MIN_POSITIVE, 1.0),
                ("small near", 0.001, 1.0),
                ("large near", 100.0, 200.0),
                ("small distance between planes", 1.0, 1.001),
                ("large far", 0.1, 10_000.0),
                ("maximum finite far", 0.1, f32::MAX),
            ];

            for (name, near, far) in cases {
                let result = Projection::orthographic(
                    DEFAULT_LEFT,
                    DEFAULT_RIGHT,
                    DEFAULT_BOTTOM,
                    DEFAULT_TOP,
                    near,
                    far,
                );

                assert_valid(name, result);
            }
        }

        #[test]
        fn rejects_invalid_horizontal_bounds() {
            let cases = [
                ("equal bounds", 1.0, 1.0),
                ("reversed bounds", 2.0, 1.0),
                ("NaN left", f32::NAN, 1.0),
                ("NaN right", -1.0, f32::NAN),
                ("infinite left", f32::INFINITY, 1.0),
                ("infinite right", -1.0, f32::INFINITY),
                ("negative infinite left", f32::NEG_INFINITY, 1.0),
                ("negative infinite right", -1.0, f32::NEG_INFINITY),
            ];

            for (name, left, right) in cases {
                let result = Projection::orthographic(
                    left,
                    right,
                    DEFAULT_BOTTOM,
                    DEFAULT_TOP,
                    DEFAULT_NEAR,
                    DEFAULT_FAR,
                );

                assert_invalid(name, result, ProjectionError::InvalidOrthographicBounds);
            }
        }

        #[test]
        fn rejects_invalid_vertical_bounds() {
            let cases = [
                ("equal bounds", 1.0, 1.0),
                ("reversed bounds", 2.0, 1.0),
                ("NaN botom", f32::NAN, 1.0),
                ("NaN top", -1.0, f32::NAN),
                ("infinite bottom", f32::INFINITY, 1.0),
                ("infinite top", -1.0, f32::INFINITY),
                ("negative infinite bottom", f32::NEG_INFINITY, 1.0),
                ("negative infinite top", -1.0, f32::NEG_INFINITY),
            ];

            for (name, bottom, top) in cases {
                let result = Projection::orthographic(
                    DEFAULT_LEFT,
                    DEFAULT_RIGHT,
                    bottom,
                    top,
                    DEFAULT_NEAR,
                    DEFAULT_FAR,
                );

                assert_invalid(name, result, ProjectionError::InvalidOrthographicBounds);
            }
        }

        #[test]
        fn rejects_invalid_planes() {
            let cases = [
                ("zero near", 0.0, 100.0),
                ("negative near", -0.1, 100.0),
                ("equal planes", 1.0, 1.0),
                ("reversed planes", 2.0, 1.0),
                ("NaN near", f32::NAN, 100.0),
                ("infinite near", f32::INFINITY, 100.0),
                ("NaN far", 0.1, f32::NAN),
                ("infinite far", 0.1, f32::INFINITY),
                ("negative infinite near", f32::NEG_INFINITY, 100.0),
                ("negtive infinite far", 0.1, f32::NEG_INFINITY),
            ];

            for (name, near, far) in cases {
                let result = Projection::orthographic(
                    DEFAULT_LEFT,
                    DEFAULT_RIGHT,
                    DEFAULT_BOTTOM,
                    DEFAULT_TOP,
                    near,
                    far,
                );

                assert_invalid(name, result, ProjectionError::InvalidOrthographicBounds);
            }
        }

        #[test]
        fn creates_correct_matrix() {
            let projection = Projection::orthographic(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0).unwrap();

            let got = projection.matrix();

            let want = Mat4::from_columns([
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, -0.2222222, 0.0),
                Vec4::new(0.0, 0.0, -1.2222222, 1.0),
            ]);

            assert_mat4_approx_eq(got, want);
        }

        #[test]
        fn maps_near_and_far_to_ndc() {
            let projection = Projection::orthographic(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0).unwrap();

            let matrix = projection.matrix();

            let near_clip = matrix * Vec4::new(0.0, 0.0, -1.0, 1.0);
            let far_clip = matrix * Vec4::new(0.0, 0.0, -10.0, 1.0);

            assert!(approx_eq(near_clip.z() / near_clip.w(), -1.0));
            assert!(approx_eq(far_clip.z() / far_clip.w(), 1.0));
        }

        #[test]
        fn preserves_parameters() {
            let projection = Projection::orthographic(-1.0, 1.0, -1.0, 1.0, 1.0, 10.0).unwrap();

            assert_eq!(
                projection.orthographic_parameters(),
                Some((-1.0, 1.0, -1.0, 1.0, 1.0, 10.0)),
            );

            assert_eq!(projection.perspective_parameters(), None);
        }
    }

    mod perspective {
        use std::f32::consts::{FRAC_PI_2, PI};

        use super::*;

        const DEFAULT_FOV: f32 = FRAC_PI_2;
        const DEFAULT_ASPECT: f32 = 1.0;
        const DEFAULT_NEAR: f32 = 0.1;
        const DEFAULT_FAR: f32 = 100.0;

        #[test]
        fn accepts_valid_fov() {
            let cases = [
                ("fov close to zero", f32::EPSILON),
                ("fov close to PI", PI - f32::EPSILON),
                ("narrow fov", 0.01),
                ("wide fov", 3.0),
            ];

            for (name, fov) in cases {
                assert_valid(
                    name,
                    Projection::perspective(
                        Angle::from_radians(fov),
                        DEFAULT_ASPECT,
                        DEFAULT_NEAR,
                        DEFAULT_FAR,
                    ),
                );
            }
        }

        #[test]
        fn accepts_valid_aspect() {
            let cases = [
                ("landscape viewport", 16.0 / 9.0),
                ("portrait viewport", 9.0 / 16.0),
                ("square viewport", 1.0),
                ("smallest positive aspect", f32::MIN_POSITIVE),
                ("very small aspect", 0.001),
                ("very large aspect", 10_000.0),
                ("maximum finite aspect", f32::MAX),
            ];

            for (name, aspect) in cases {
                assert_valid(
                    name,
                    Projection::perspective(
                        Angle::from_radians(DEFAULT_FOV),
                        aspect,
                        DEFAULT_NEAR,
                        DEFAULT_FAR,
                    ),
                );
            }
        }

        #[test]
        fn accepts_valid_planes() {
            let cases = [
                ("smallest positive near", f32::MIN_POSITIVE, 1.0),
                ("small near", 0.001, 1.0),
                ("large near", 100.0, 200.0),
                ("small distance between planes", 1.0, 1.001),
                ("large far", 0.1, 10_000.0),
                ("maximum finite far", 0.1, f32::MAX),
            ];

            for (name, near, far) in cases {
                assert_valid(
                    name,
                    Projection::perspective(
                        Angle::from_radians(DEFAULT_FOV),
                        DEFAULT_ASPECT,
                        near,
                        far,
                    ),
                );
            }
        }

        #[test]
        fn rejects_invalid_fov() {
            let cases = [
                ("zero fov", 0.0),
                ("negative fov", -1.0),
                ("fov equal to PI", std::f32::consts::PI),
                ("fov greater than PI", std::f32::consts::PI + 0.1),
                ("NaN fov", f32::NAN),
                ("infinite fov", f32::INFINITY),
            ];

            for (name, fov) in cases {
                assert_invalid(
                    name,
                    Projection::perspective(
                        Angle::from_radians(fov),
                        DEFAULT_ASPECT,
                        DEFAULT_NEAR,
                        DEFAULT_FAR,
                    ),
                    ProjectionError::InvalidFieldOfView,
                );
            }
        }

        #[test]
        fn rejects_invalid_aspect() {
            let cases = [
                ("zero aspect", 0.0),
                ("negative aspect", -1.0),
                ("NaN aspect", f32::NAN),
                ("infinite aspect", f32::INFINITY),
            ];

            for (name, aspect) in cases {
                assert_invalid(
                    name,
                    Projection::perspective(
                        Angle::from_radians(DEFAULT_FOV),
                        aspect,
                        DEFAULT_NEAR,
                        DEFAULT_FAR,
                    ),
                    ProjectionError::InvalidAspectRatio,
                );
            }
        }

        #[test]
        fn rejects_invalid_planes() {
            let cases = [
                ("zero near plane", 0.0, 100.0),
                ("negative near plane", -0.1, 100.0),
                ("near equal to far", 1.0, 1.0),
                ("near greater than far", 2.0, 1.0),
                ("NaN near plane", f32::NAN, 100.0),
                ("infinite near plane", f32::INFINITY, 100.0),
                ("NaN far plane", 0.1, f32::NAN),
                ("infinite far plane", 0.1, f32::INFINITY),
            ];

            for (name, near, far) in cases {
                assert_invalid(
                    name,
                    Projection::perspective(
                        Angle::from_radians(DEFAULT_FOV),
                        DEFAULT_ASPECT,
                        near,
                        far,
                    ),
                    ProjectionError::InvalidClippingPlanes,
                );
            }
        }

        #[test]
        fn rejects_unrepresentable_matrix() {
            let result = Projection::perspective(
                Angle::from_degrees(10.0),
                f32::MIN_POSITIVE,
                DEFAULT_NEAR,
                DEFAULT_FAR,
            );

            assert_invalid(
                "min positive aspect",
                result,
                ProjectionError::UnrepresentableMatrix,
            );
        }

        #[test]
        fn creates_correct_matrix() {
            let projection =
                Projection::perspective(Angle::from_radians(FRAC_PI_2), 1.0, 1.0, 10.0).unwrap();

            let got = projection.matrix();

            let want = Mat4::from_columns([
                Vec4::new(1.0, 0.0, 0.0, 0.0),
                Vec4::new(0.0, 1.0, 0.0, 0.0),
                Vec4::new(0.0, 0.0, -1.2222222, -1.0),
                Vec4::new(0.0, 0.0, -2.2222222, 0.0),
            ]);

            assert_mat4_approx_eq(got, want);
        }

        #[test]
        fn maps_near_and_far_to_ndc() {
            let projection =
                Projection::perspective(Angle::from_radians(FRAC_PI_2), 1.0, 1.0, 10.0).unwrap();

            let matrix = projection.matrix();

            let near_clip = matrix * Vec4::new(0.0, 0.0, -1.0, 1.0);
            let far_clip = matrix * Vec4::new(0.0, 0.0, -10.0, 1.0);

            assert!(approx_eq(near_clip.z() / near_clip.w(), -1.0));
            assert!(approx_eq(far_clip.z() / far_clip.w(), 1.0));
        }

        #[test]
        fn preserves_parameters() {
            let fov = Angle::from_radians(FRAC_PI_2);

            let projection = Projection::perspective(fov, 16.0 / 9.0, 0.1, 100.0).unwrap();

            assert_eq!(
                projection.perspective_parameters(),
                Some((fov, 16.0 / 9.0, 0.1, 100.0)),
            );

            assert_eq!(projection.orthographic_parameters(), None);
        }
    }

    fn assert_valid(case: &str, result: Result<Projection, ProjectionError>) {
        let projection = result.unwrap_or_else(|err| {
            panic!("Test case '{case}' should produce a projection, but got {err:?}");
        });

        assert!(
            projection.matrix().is_finite(),
            "Test case '{case}' should contain a finite projection matrix"
        );
    }

    fn assert_invalid(
        case: &str,
        result: Result<Projection, ProjectionError>,
        want_err: ProjectionError,
    ) {
        let err = result.expect_err(&format!(
            "Test case '{case}': expected an error, but got Ok"
        ));

        assert_eq!(
            err,
            want_err,
            "Test case '{case}': unexpected projection error ({:?})",
            result.unwrap_err()
        );
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
}
