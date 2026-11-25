use crate::math::Vec3;

/// Errors returnedwhen creating or resizing a viewport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewportError {
    /// ViewPort dimensions must both be greater than zero.
    InvalidDimensions { width: u32, height: u32 },
}

/// Represents a rectangular viewport in screen space.
///
/// Both dimensions are guaranteed to be strictly positive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    width: u32,
    height: u32,
}

impl Viewport {
    /// Creates a viewport with validated dimesions.
    pub fn new(width: u32, height: u32) -> Result<Self, ViewportError> {
        if width == 0 || height == 0 {
            return Err(ViewportError::InvalidDimensions { width, height });
        }

        Ok(Self { width, height })
    }

    /// Returns the viewport width in pixels.
    pub const fn width(self) -> u32 {
        self.width
    }

    /// Returns the viewport height in pixels.
    pub const fn height(self) -> u32 {
        self.height
    }

    /// Returns the viewport aspect ratio.
    pub fn aspect_ratio(self) -> f32 {
        self.width as f32 / self.height as f32
    }

    /// Updates viewport dimensions.
    ///
    /// The previous state is preserved if the dimensions are invalid.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), ViewportError> {
        if width == 0 || height == 0 {
            return Err(ViewportError::InvalidDimensions { width, height });
        }

        self.width = width;
        self.height = height;

        Ok(())
    }

    /// Converts normalized device coordinates into screen coordinates.
    ///
    /// Screen coordinates use a top-left origin.
    /// X increases to the right and Y increases downward.
    /// The depth component is preserved.
    ///
    /// Screen coordinates represent continuous pixel-edge coordinates.
    /// with the viewport covering [0, width] x [0, height].
    pub fn ndc_to_screen(self, ndc: Vec3) -> Vec3 {
        let half_width = self.width as f32 * 0.5;
        let half_heigth = self.height as f32 * 0.5;

        Vec3::new(
            (ndc.x() + 1.0) * half_width,
            (1.0 - ndc.y()) * half_heigth,
            ndc.z(),
        )
    }

    /// Converts screen coordinatesinto normalized device coordinates.
    ///
    /// Screen coordinates use a top-left origin.
    /// The depth component is preserved.
    pub fn screen_to_ndc(self, screen: Vec3) -> Vec3 {
        let inverse_half_width = 2.0 / self.width as f32;
        let inverse_half_height = 2.0 / self.height as f32;

        Vec3::new(
            screen.x() * inverse_half_width - 1.0,
            1.0 - screen.y() * inverse_half_height,
            screen.z(),
        )
    }

    /// Converts a scene-space displacement into an NDC displacement.
    ///
    /// Positive screen Y points downward, while positive NDC Y points upward.
    pub fn screen_delta_to_ndc(self, dx: f32, dy: f32) -> Vec3 {
        Vec3::new(
            2.0 * dx / self.width as f32,
            -2.0 * dy / self.height as f32,
            0.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn construction_rejects_zero_dimensions() {
        let cases = [
            (
                "zero width",
                0,
                600,
                ViewportError::InvalidDimensions {
                    width: 0,
                    height: 600,
                },
            ),
            (
                "zero height",
                800,
                0,
                ViewportError::InvalidDimensions {
                    width: 800,
                    height: 0,
                },
            ),
            (
                "both zero",
                0,
                0,
                ViewportError::InvalidDimensions {
                    width: 0,
                    height: 0,
                },
            ),
        ];

        for (name, w, h, want_err) in cases {
            let err = Viewport::new(w, h)
                .expect_err(&format!("Test case '{name}' expect an error, but got Ok"));

            assert_eq!(
                err, want_err,
                "Test case '{name}': unexpected viewport error ({err:?})"
            );
        }
    }

    #[test]
    fn aspect_ration_calculation() {
        let cases = [
            ("landscape viewport", 1920, 1080, 1.7777778),
            ("portrait viewport", 600, 800, 0.75),
            ("square viewport", 1024, 1024, 1.0),
            ("widescreen viewport", 2560, 1080, 2.3703704),
        ];

        for (name, w, h, want) in cases {
            let viewport = Viewport::new(w, h).unwrap();

            let got = viewport.aspect_ratio();

            assert!(
                approx_eq(got, want),
                "Test case '{name}' aspect ration mismatch: got {got}, want {want}"
            );
        }
    }

    #[test]
    fn resize_updates_dimensions() {
        let mut viewport = Viewport::new(800, 600).unwrap();

        viewport.resize(1920, 1080).unwrap();

        assert_eq!(viewport.width(), 1920);
        assert_eq!(viewport.height(), 1080);
        assert_eq!(viewport.aspect_ratio(), 1.7777778);
    }

    #[test]
    fn resize_rejects_zero_dimensions_and_preserves_state() {
        let mut viewport = Viewport::new(1920, 1080).unwrap();

        let cases = [
            (
                "zero width",
                0,
                600,
                ViewportError::InvalidDimensions {
                    width: 0,
                    height: 600,
                },
            ),
            (
                "zero height",
                800,
                0,
                ViewportError::InvalidDimensions {
                    width: 800,
                    height: 0,
                },
            ),
            (
                "both zero",
                0,
                0,
                ViewportError::InvalidDimensions {
                    width: 0,
                    height: 0,
                },
            ),
        ];

        for (name, w, h, want_err) in cases {
            let err = viewport
                .resize(w, h)
                .expect_err(&format!("Test case '{name}' expect an error, but got Ok"));

            assert_eq!(
                err, want_err,
                "Test case '{name}': unexpected viewport error ({err:?})"
            );
        }
    }

    #[test]
    fn ndc_to_scren_maps_corners_and_center() {
        let viewport = Viewport::new(800, 600).unwrap();

        let cases = [
            (
                "top-left corner",
                Vec3::new(-1.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ),
            (
                "top-right corner",
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(800.0, 0.0, 0.0),
            ),
            (
                "bottom-left corner",
                Vec3::new(-1.0, -1.0, 0.0),
                Vec3::new(0.0, 600.0, 0.0),
            ),
            (
                "bottom-right corner",
                Vec3::new(1.0, -1.0, 0.0),
                Vec3::new(800.0, 600.0, 0.0),
            ),
            (
                "center",
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(400.0, 300.0, 0.0),
            ),
        ];

        for (name, ndc, want) in cases {
            assert_vec3_approx_eq(viewport.ndc_to_screen(ndc), want, name);
        }
    }

    #[test]
    fn ndc_to_screen_preserves_depth() {
        let viewport = Viewport::new(800, 600).unwrap();

        let result = viewport.ndc_to_screen(Vec3::new(0.5, -0.5, 0.75));

        assert_eq!(result.z(), 0.75);
    }

    #[test]
    fn screen_to_ndc_maps_corners_and_center() {
        let viewport = Viewport::new(800, 600).unwrap();

        let cases = [
            (
                "top-left corner",
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(-1.0, 1.0, 0.0),
            ),
            (
                "top-right corner",
                Vec3::new(800.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
            ),
            (
                "bottom-left corner",
                Vec3::new(0.0, 600.0, 0.0),
                Vec3::new(-1.0, -1.0, 0.0),
            ),
            (
                "bottom-right corner",
                Vec3::new(800.0, 600.0, 0.0),
                Vec3::new(1.0, -1.0, 0.0),
            ),
            (
                "center",
                Vec3::new(400.0, 300.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ),
        ];

        for (name, ndc, want) in cases {
            assert_vec3_approx_eq(viewport.screen_to_ndc(ndc), want, name);
        }
    }

    #[test]
    fn ndc_screen_round_trip() {
        let viewport = Viewport::new(800, 600).unwrap();

        let cases = [
            ("top-left corner", Vec3::new(-1.0, 1.0, 0.0)),
            ("top-right corner", Vec3::new(1.0, 1.0, 0.0)),
            ("bottom-left corner", Vec3::new(-1.0, -1.0, 0.0)),
            ("bottom-right corner", Vec3::new(1.0, -1.0, 0.0)),
            ("center", Vec3::new(0.0, 0.0, 0.0)),
        ];

        for (name, ndc) in cases {
            let screen = viewport.ndc_to_screen(ndc);
            let restored = viewport.screen_to_ndc(screen);

            assert_vec3_approx_eq(restored, ndc, name);
        }
    }

    #[test]
    fn screen_delta_to_ndc_converts_mouse_displacement() {
        let viewport = Viewport::new(800, 600).unwrap();

        let cases = [
            ("positive delta", 400.0, 300.0, Vec3::new(1.0, -1.0, 0.0)),
            ("negative delta", -400.0, -300.0, Vec3::new(-1.0, 1.0, 0.0)),
            ("zero delta", 0.0, 0.0, Vec3::ZERO),
        ];

        for (name, dx, dy, want) in cases {
            let got = viewport.screen_delta_to_ndc(dx, dy);

            assert_vec3_approx_eq(got, want, name);
        }
    }

    fn assert_vec3_approx_eq(got: Vec3, want: Vec3, ctx: &str) {
        let prefix = if ctx.is_empty() {
            String::new()
        } else {
            format!("[{ctx}] ")
        };

        assert!(
            approx_eq(got.x(), want.x())
                && approx_eq(got.y(), want.y())
                && approx_eq(got.z(), want.z()),
            "{prefix}Vec3 assertion failed (epsilon = {EPSILON})\n {:>4}: {got:?}\n {:>4}: {want:?}",
            "got",
            "want",
        )
    }

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() <= EPSILON
    }
}
