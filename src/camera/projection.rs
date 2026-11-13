use crate::math::{Mat4, Vec4};

/// Creates a right-handed orthographic projection matrix.
///
/// The resulting normalized device coordinate depth range is [-1, 1].
pub fn orthographic(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    near: f32,
    far: f32,
) -> Option<Mat4> {
    if !left.is_finite()
        || !right.is_finite()
        || !bottom.is_finite()
        || !top.is_finite()
        || !near.is_finite()
        || !far.is_finite()
        || left >= right
        || bottom >= top
        || near <= 0.0
        || near >= far
    {
        return None;
    }

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

    if [x_scale, y_scale, z_scale, x_offset, y_offset, z_offset]
        .iter()
        .any(|val| !val.is_finite() || *val < f64::from(f32::MIN) || *val > f64::from(f32::MAX))
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

/// Crates a right-handed perspective projection matrix.
///
/// The resulting normalized device coordinate depth range is [-1, 1].
pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Option<Mat4> {
    if !fov_y.is_finite()
        || !aspect.is_finite()
        || !near.is_finite()
        || !far.is_finite()
        || fov_y <= 0.0
        || fov_y >= std::f32::consts::PI
        || aspect <= 0.0
        || near <= 0.0
        || near >= far
    {
        return None;
    }

    let fov_y = f64::from(fov_y);
    let aspect_ratio = f64::from(aspect);
    let near = f64::from(near);
    let far = f64::from(far);

    let focal_length = 1.0 / (fov_y * 0.5).tan();
    let depth = far - near;

    let x_scale = focal_length / aspect_ratio;
    let y_scale = focal_length;
    let z_scale = -(far + near) / depth;
    let z_offset = -(2.0 * far * near) / depth;

    if [x_scale, y_scale, z_scale, z_offset]
        .iter()
        .any(|val| !val.is_finite() || *val < f64::from(f32::MIN) || *val > f64::from(f32::MAX))
    {
        return None;
    }

    Some(Mat4::from_columns([
        Vec4::new(x_scale as f32, 0.0, 0.0, 0.0),
        Vec4::new(0.0, y_scale as f32, 0.0, 0.0),
        Vec4::new(0.0, 0.0, z_scale as f32, -1.0),
        Vec4::new(0.0, 0.0, 0.0, z_offset as f32),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod orthographic {
        use super::super::*;
        use crate::camera::projection;

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
                assert_valid(
                    name,
                    projection::orthographic(
                        left,
                        right,
                        DEFAULT_BOTTOM,
                        DEFAULT_TOP,
                        DEFAULT_NEAR,
                        DEFAULT_FAR,
                    ),
                );
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
                assert_valid(
                    name,
                    projection::orthographic(
                        DEFAULT_LEFT,
                        DEFAULT_RIGHT,
                        bottom,
                        top,
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
                    projection::orthographic(
                        DEFAULT_LEFT,
                        DEFAULT_RIGHT,
                        DEFAULT_BOTTOM,
                        DEFAULT_TOP,
                        near,
                        far,
                    ),
                );
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
                assert_invalid(
                    name,
                    projection::orthographic(
                        left,
                        right,
                        DEFAULT_BOTTOM,
                        DEFAULT_TOP,
                        DEFAULT_NEAR,
                        DEFAULT_FAR,
                    ),
                );
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
                assert_invalid(
                    name,
                    projection::orthographic(
                        DEFAULT_LEFT,
                        DEFAULT_RIGHT,
                        bottom,
                        top,
                        DEFAULT_NEAR,
                        DEFAULT_FAR,
                    ),
                );
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
                assert_invalid(
                    name,
                    projection::orthographic(
                        DEFAULT_LEFT,
                        DEFAULT_RIGHT,
                        DEFAULT_BOTTOM,
                        DEFAULT_TOP,
                        near,
                        far,
                    ),
                );
            }
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
    }

    mod perspective {
        use super::*;
        use crate::camera::projection;

        const DEFAULT_FOV: f32 = 1.0;
        const DEFAULT_ASPECT: f32 = 1.0;
        const DEFAULT_NEAR: f32 = 0.1;
        const DEFAULT_FAR: f32 = 100.0;

        #[test]
        fn accepts_valid_fov() {
            let cases = [
                ("fov close to zero", f32::EPSILON),
                ("fov close to PI", std::f32::consts::PI - f32::EPSILON),
                ("narrow fov", 0.01),
                ("wide fov", 3.0),
            ];

            for (name, fov) in cases {
                assert_valid(
                    name,
                    projection::perspective(fov, DEFAULT_ASPECT, DEFAULT_NEAR, DEFAULT_FAR),
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
                    projection::perspective(DEFAULT_FOV, aspect, DEFAULT_NEAR, DEFAULT_FAR),
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
                    projection::perspective(DEFAULT_FOV, DEFAULT_ASPECT, near, far),
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
                    projection::perspective(fov, DEFAULT_ASPECT, DEFAULT_NEAR, DEFAULT_FAR),
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
                    projection::perspective(DEFAULT_FOV, aspect, DEFAULT_NEAR, DEFAULT_FAR),
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
                    projection::perspective(DEFAULT_FOV, DEFAULT_ASPECT, near, far),
                );
            }
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
    }
}
