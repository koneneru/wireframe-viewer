use crate::math::{Mat4, Vec3, Vec4};

pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Option<Mat4> {
    if !eye.is_finite() {
        return None;
    }

    let forward = (target - eye).try_normalize()?;
    let side = forward.cross(up).try_normalize()?;
    let up = side.cross(forward);

    let view = Mat4::from_columns([
        Vec4::new(side.x(), up.x(), -forward.x(), 0.0),
        Vec4::new(side.y(), up.y(), -forward.y(), 0.0),
        Vec4::new(side.z(), up.z(), -forward.z(), 0.0),
        Vec4::new(-side.dot(eye), -up.dot(eye), forward.dot(eye), 1.0),
    ]);

    view.is_finite().then_some(view)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod look_at {
        use super::*;
        use crate::camera::view;

        const EPSILON: f32 = 1e-6;

        const DEFAULT_EYE: Vec3 = Vec3::ZERO;
        const DEFAULT_TARGET: Vec3 = Vec3::new(0.0, 0.0, -1.0);
        const DEFAULT_UP: Vec3 = Vec3::Y;

        #[test]
        fn accepts_valid_camera_position() {
            let cases = [
                ("camera at origin", Vec3::ZERO, Vec3::Z),
                ("positive position", Vec3::new(1.0, 2.0, 3.0), Vec3::ZERO),
                ("negative position", Vec3::new(-1.0, -2.0, -3.0), Vec3::ZERO),
                (
                    "large position",
                    Vec3::new(10_000.0, 10_000.0, 10_000.0),
                    Vec3::ZERO,
                ),
                ("small distance", Vec3::ZERO, Vec3::new(0.0, 0.0, 0.001)),
            ];

            for (name, eye, target) in cases {
                assert_valid(name, view::look_at(eye, target, Vec3::Y));
            }
        }

        #[test]
        fn accepts_valid_up_vector() {
            let cases = [
                ("positive Y up", Vec3::Y),
                ("negative Y up", -Vec3::Y),
                ("positive X up", Vec3::X),
                ("negative X up", -Vec3::X),
                ("non-unit up", Vec3::new(0.0, 10.0, 0.0)),
                ("diagonal up", Vec3::new(1.0, 1.0, 0.0)),
            ];

            for (name, up) in cases {
                assert_valid(name, view::look_at(DEFAULT_EYE, DEFAULT_TARGET, up));
            }
        }

        #[test]
        fn rejects_invalid_eye() {
            let cases = [
                ("NaN X", Vec3::new(f32::NAN, 0.0, 0.0)),
                ("NaN Y", Vec3::new(0.0, f32::NAN, 0.0)),
                ("NaN Z", Vec3::new(0.0, 0.0, f32::NAN)),
                ("infinite X", Vec3::new(f32::INFINITY, 0.0, 0.0)),
                ("infinite Y", Vec3::new(0.0, f32::INFINITY, 0.0)),
                ("infinite Z", Vec3::new(0.0, 0.0, f32::INFINITY)),
                (
                    "negative infinite X",
                    Vec3::new(f32::NEG_INFINITY, 0.0, 0.0),
                ),
                (
                    "negative infinite Y",
                    Vec3::new(0.0, f32::NEG_INFINITY, 0.0),
                ),
                (
                    "negative infinite Z",
                    Vec3::new(0.0, 0.0, f32::NEG_INFINITY),
                ),
            ];

            for (name, eye) in cases {
                assert_invalid(name, view::look_at(eye, DEFAULT_TARGET, DEFAULT_UP));
            }
        }

        #[test]
        fn rejects_invalid_target() {
            let cases = [
                ("target equals eye", DEFAULT_EYE),
                ("NaN X", Vec3::new(f32::NAN, 0.0, 0.0)),
                ("NaN Y", Vec3::new(0.0, f32::NAN, 0.0)),
                ("NaN Z", Vec3::new(0.0, 0.0, f32::NAN)),
                ("infinite X", Vec3::new(f32::INFINITY, 0.0, 0.0)),
                ("infinite Y", Vec3::new(0.0, f32::INFINITY, 0.0)),
                ("infinite Z", Vec3::new(0.0, 0.0, f32::INFINITY)),
                (
                    "negative infinite X",
                    Vec3::new(f32::NEG_INFINITY, 0.0, 0.0),
                ),
                (
                    "negative infinite Y",
                    Vec3::new(0.0, f32::NEG_INFINITY, 0.0),
                ),
                (
                    "negative infinite Z",
                    Vec3::new(0.0, 0.0, f32::NEG_INFINITY),
                ),
            ];

            for (name, target) in cases {
                assert_invalid(name, view::look_at(DEFAULT_EYE, target, DEFAULT_UP));
            }
        }

        #[test]
        fn rejects_invalid_up() {
            let cases = [
                ("zero up", Vec3::ZERO),
                ("NaN X", Vec3::new(f32::NAN, 0.0, 0.0)),
                ("NaN Y", Vec3::new(0.0, f32::NAN, 0.0)),
                ("NaN Z", Vec3::new(0.0, 0.0, f32::NAN)),
                ("infinite X", Vec3::new(f32::INFINITY, 0.0, 0.0)),
                ("infinite Y", Vec3::new(0.0, f32::INFINITY, 0.0)),
                ("infinite Z", Vec3::new(0.0, 0.0, f32::INFINITY)),
                (
                    "negative infinite X",
                    Vec3::new(f32::NEG_INFINITY, 0.0, 0.0),
                ),
                (
                    "negative infinite Y",
                    Vec3::new(0.0, f32::NEG_INFINITY, 0.0),
                ),
                (
                    "negative infinite Z",
                    Vec3::new(0.0, 0.0, f32::NEG_INFINITY),
                ),
            ];

            for (name, up) in cases {
                assert_invalid(name, view::look_at(DEFAULT_EYE, DEFAULT_TARGET, up));
            }
        }

        #[test]
        fn rejects_up_parallel_to_forward() {
            let cases = [
                ("parallel", Vec3::new(0.0, 0.0, 2.0)),
                ("anti-parallel", Vec3::new(0.0, 0.0, -2.0)),
            ];

            for (name, up) in cases {
                assert_invalid(name, view::look_at(DEFAULT_EYE, DEFAULT_TARGET, up));
            }
        }

        #[test]
        fn returns_identity_for_default_camera_orientation() {
            let got = view::look_at(DEFAULT_EYE, DEFAULT_TARGET, DEFAULT_UP);
            let want = Mat4::IDENTITY;

            assert_valid("default camera orientation", got);

            assert_eq!(got.unwrap(), want)
        }

        #[test]
        fn transforms_eye_to_origin() {
            let cases = [
                ("positive position", Vec3::new(1.0, 2.0, 3.0)),
                ("negative position", Vec3::new(-1.0, -2.0, -3.0)),
                ("large position", Vec3::new(100.0, 200.0, 300.0)),
            ];

            for (name, eye) in cases {
                let target = eye + Vec3::new(0.0, 0.0, -1.0);

                let result = view::look_at(eye, target, DEFAULT_UP);
                assert_valid(name, result);

                let got = result.unwrap();

                let transformed_eye = got * Vec4::new(eye.x(), eye.y(), eye.z(), 1.0);

                assert_eq!(transformed_eye, Vec4::new(0.0, 0.0, 0.0, 1.0))
            }
        }

        #[test]
        fn look_at_rotation() {
            let got = view::look_at(DEFAULT_EYE, Vec3::X, DEFAULT_UP).unwrap();

            let expected = Mat4::rotation_y(std::f32::consts::FRAC_PI_2);

            assert_mat4_approx_eq(got, expected);
        }

        #[test]
        fn look_at_transforms_target_to_negative_z() {
            let eye = Vec3::new(0.0, 0.0, 5.0);
            let target = Vec3::ZERO;

            let view = view::look_at(eye, target, DEFAULT_UP).unwrap();

            assert_vec4_approx_eq(
                view * Vec4::new(target.x(), target.y(), target.z(), 1.0),
                Vec4::new(0.0, 0.0, -5.0, 1.0),
            );
        }

        fn assert_valid(case: &str, result: Option<Mat4>) {
            let matrix =
                result.unwrap_or_else(|| panic!("Test case '{case}' should produce a matrix"));

            assert!(
                matrix.is_finite(),
                "Test case '{case}' should produce a finite matrix"
            );
        }

        fn assert_invalid(case: &str, result: Option<Mat4>) {
            assert!(
                result.is_none(),
                "Test case '{case}' should reject invalid matrix"
            );
        }

        fn approx_eq(a: f32, b: f32) -> bool {
            (a - b).abs() <= EPSILON
        }

        fn assert_vec4_approx_eq(got: Vec4, want: Vec4) {
            let components = [
                ("x", got.x(), want.x()),
                ("y", got.y(), want.y()),
                ("z", got.z(), want.z()),
                ("w", got.w(), want.w()),
            ];

            for (name, got, want) in components {
                assert!(
                    approx_eq(got, want),
                    "Vector mismatch at {name}: got {got}, want {want}",
                );
            }
        }

        fn assert_mat4_approx_eq(got: Mat4, expected: Mat4) {
            for col in 0..4 {
                for row in 0..4 {
                    let got = got[(row, col)];
                    let expected = expected[(row, col)];

                    assert!(
                        approx_eq(got, expected),
                        "Matrix mismatch at row {row}, column {col}: got {got:?}, excepted {expected:?}"
                    );
                }
            }
        }
    }
}
