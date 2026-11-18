use std::{
    f32::consts::{FRAC_PI_2, PI, TAU},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// Represents an angle normalized to the range [-PI, PI).
///
/// Non-finite input values are converted to zero.
/// All arithmetic operations preserve the normalization invariant.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle {
    radians: f32,
}

impl Angle {
    /// The zero angle.
    pub const ZERO: Self = Self { radians: 0.0 };

    /// A half-turn angle.
    pub const HALF_TURN: Self = Self { radians: -PI };

    /// A quater-turn angle.
    pub const QUARTER_TURN: Self = Self { radians: FRAC_PI_2 };

    /// Creates an angle from radians and normalizes it.
    pub fn from_radians(radians: f32) -> Self {
        Self::new(radians)
    }

    /// Creates an angle from degrees and normalizes it.
    pub fn from_degrees(degrees: f32) -> Self {
        Self::new(degrees.to_radians())
    }

    /// Returns the angle in radians.
    pub fn as_radians(self) -> f32 {
        self.radians
    }

    /// Returns the angle in degrees.
    pub fn as_degrees(self) -> f32 {
        self.radians.to_degrees()
    }

    /// Returns the sine of the angle.
    pub fn sin(self) -> f32 {
        self.radians.sin()
    }

    /// Returns the cosine of the angle.
    pub fn cos(self) -> f32 {
        self.radians.cos()
    }

    /// Returns the sine and cosine of the angle.
    pub fn sin_cos(self) -> (f32, f32) {
        self.radians.sin_cos()
    }

    /// Creates a normalized angle.
    fn new(radians: f32) -> Self {
        Self {
            radians: Self::normalize(radians),
        }
    }

    /// Normalizes radians to the range [-PI, PI).
    ///
    /// Non-finite values are mapped to zero.
    fn normalize(radians: f32) -> f32 {
        if !radians.is_finite() {
            return 0.0;
        }

        let normalized = (radians + PI).rem_euclid(TAU) - PI;

        if normalized == 0.0 { 0.0 } else { normalized }
    }
}

// Angle addition

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.radians + rhs.radians)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.radians + rhs.radians);
    }
}

// Angle subtraction

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.radians - rhs.radians)
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self.radians - rhs.radians);
    }
}

// Scalar multiplication

impl Mul<f32> for Angle {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.radians * rhs)
    }
}

impl MulAssign<f32> for Angle {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Self::new(self.radians * rhs)
    }
}

// Negation

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.radians)
    }
}

// Scalar multiplication from the left

impl Mul<Angle> for f32 {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    mod ctor {
        use super::*;

        #[test]
        fn from_radians() {
            let cases = [
                ("zero", 0.0, 0.0),
                ("negative zero", -0.0, 0.0),
                ("PI / 2", FRAC_PI_2, FRAC_PI_2),
                ("PI", PI, -PI),
                ("3 * PI / 2", 3.0 * FRAC_PI_2, -FRAC_PI_2),
                ("TAU", TAU, 0.0),
                ("negative PI / 2", -FRAC_PI_2, -FRAC_PI_2),
                ("negative PI", -PI, -PI),
                ("-3 * PI / 2", -3.0 * FRAC_PI_2, FRAC_PI_2),
                ("negative TAU", -TAU, 0.0),
                ("4 * PI", 4.0 * PI, 0.0),
                ("-6 * PI", -6.0 * PI, 0.0),
            ];

            for (name, input, want) in cases {
                let angle = Angle::from_radians(input);
                let got = angle.as_radians();

                assert!(
                    approx_eq(got, want),
                    "Test case '{name}' failed: got {got}, want {want}"
                );
            }
        }

        #[test]
        fn from_degrees() {
            let cases = [
                ("zero", 0.0, 0.0),
                ("negative zero", -0.0, 0.0),
                ("90 degrees", 90.0, FRAC_PI_2),
                ("180 degrees", 180.0, -PI),
                ("270 degrees", 270.0, -FRAC_PI_2),
                ("360 degrees", 360.0, 0.0),
                ("-90 degrees", -90.0, -FRAC_PI_2),
                ("-180 degrees", -180.0, -PI),
                ("-270 degrees", -270.0, FRAC_PI_2),
                ("-360 degrees", -360.0, 0.0),
                ("720 degrees", 720.0, 0.0),
                ("1080 degrees", -1080.0, 0.0),
            ];

            for (name, input, want) in cases {
                let angle = Angle::from_degrees(input);
                let got = angle.as_radians();

                assert!(
                    approx_eq(got, want),
                    "Test case '{name}' failed: got {got}, want {want}",
                );
            }
        }

        #[test]
        fn normalizes_lerge_angle_to_canonical_range() {
            let cases = [
                ("large positive multiply of TAU", 1000.0 * TAU),
                ("large negative multiply of TAU", -1000.0 * TAU),
                ("large positive multiply of PI/2", 1000.0 * TAU + FRAC_PI_2),
                ("large negative multiply of PI/2", -1000.0 * TAU - FRAC_PI_2),
            ];

            for (name, input) in cases {
                let angle = Angle::from_radians(input);
                let got = angle.as_radians();

                assert!(
                    got >= -PI,
                    "Test case '{name}' result less than -PI: got {got}",
                );
                assert!(
                    got < PI,
                    "Test case '{name}' result greater or equal than PI: got {got}"
                );
            }
        }

        #[test]
        fn zeroes_non_finite_input() {
            let cases = [
                ("NaN input", f32::NAN),
                ("infinite input", f32::INFINITY),
                ("negative infinite input", f32::NEG_INFINITY),
            ];

            for (name, input) in cases {
                let angle = Angle::from_radians(input);
                let got = angle.as_radians();

                assert_eq!(got, 0.0, "Test case '{name}' failed: got {got}, want 0.0");
            }
        }
    }

    mod arithmetics {
        use super::*;

        #[test]
        fn addition() {
            let cases = [
                ("default", 30.0, 45.0, 75.0),
                ("cross upper bound", 110.0, 90.0, -160.0),
                ("cross lower bound", -179.0, -2.0, 179.0),
                ("exact at lower bound from bottom", -179.0, -1.0, -180.0),
                ("exact at lower bound from top", 179.0, 1.0, -180.0),
                ("cross full circle", 270.0, 180.0, 90.0),
            ];

            for (name, lhs, rhs, want) in cases {
                let angle = Angle::from_degrees(lhs) + Angle::from_degrees(rhs);
                let want = Angle::from_degrees(want);

                assert_angle_deg_eq(name, angle, want);
            }
        }

        #[test]
        fn subtraction() {
            let cases = [
                ("default", 75.0, 30.0, 45.0),
                ("cross upper bound", 170.0, -30.0, -160.0),
                ("cross lower bound", -170.0, 30.0, 160.0),
                ("exact lower bound", 0.0, 180.0, -180.0),
                ("cross full circle", 270.0, 90.0, -180.0),
            ];

            for (name, lhs, rhs, want) in cases {
                let got = Angle::from_degrees(lhs) - Angle::from_degrees(rhs);
                let want = Angle::from_degrees(want);

                assert_angle_deg_eq(name, got, want);
            }
        }

        #[test]
        fn multiplication() {
            let cases = [
                ("default", 30.0, 2.0, 60.0),
                ("cross upper bound", 100.0, 2.0, -160.0),
                ("cross lower bound", -100.0, 2.0, 160.0),
                ("negative scalar", 45.0, -2.0, -90.0),
                ("zero scalar", 120.0, 0.0, 0.0),
                ("full turn", 120.0, 3.0, 0.0),
                ("fraction scalar", 90.0, 0.5, 45.0),
            ];

            for (name, input, scalar, want) in cases {
                let angle_from_left = Angle::from_degrees(input) * scalar;
                let angle_from_right = scalar * Angle::from_degrees(input);
                let want = Angle::from_degrees(want);

                assert_eq!(
                    angle_from_left, angle_from_right,
                    "Test case '{name}' left and right multiplication differ"
                );
                assert_angle_deg_eq(name, angle_from_left, want);
            }
        }

        #[test]
        fn negation() {
            let cases = [
                ("positive angle", 45.0, -45.0),
                ("negative angle", -45.0, 45.0),
                ("upper half turn", 179.0, -179.0),
                ("lower half turn", -180.0, 180.0),
                ("full turn", 360.0, 0.0),
            ];

            for (name, input, want) in cases {
                let got = -Angle::from_degrees(input);
                let want = Angle::from_degrees(want);

                assert_angle_deg_eq(name, got, want);
            }
        }

        #[test]
        fn addition_assign() {
            let mut angle = Angle::from_degrees(110.0);
            angle += Angle::from_degrees(90.0);

            assert_angle_deg_eq("addition assign", angle, Angle::from_degrees(-160.0));
        }

        #[test]
        fn subtraction_assign() {
            let mut angle = Angle::from_degrees(-170.0);
            angle -= Angle::from_degrees(30.0);

            assert_angle_deg_eq("subtraction assign", angle, Angle::from_degrees(160.0));
        }

        #[test]
        fn multiplication_assign() {
            let mut angle = Angle::from_degrees(100.0);
            angle *= 2.0;

            assert_angle_deg_eq("multiplication assign", angle, Angle::from_degrees(-160.0));
        }
    }

    #[test]
    fn trigonometric_functions() {
        use std::f32::consts::FRAC_1_SQRT_2;
        let sqrt3_over_2 = 3.0_f32.sqrt() / 2.0;

        let cases = [
            ("zero degrees", 0.0, 0.0, 1.0),
            ("30 degrees", 30.0, 0.5, sqrt3_over_2),
            ("45 degrees", 45.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2),
            ("60 degrees", 60.0, sqrt3_over_2, 0.5),
            ("90 degrees", 90.0, 1.0, 0.0),
            ("135 degrees", 135.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
            ("180 degrees", 180.0, 0.0, -1.0),
            ("270 degrees", 270.0, -1.0, 0.0),
            ("360 degrees", 360.0, 0.0, 1.0),
            ("450 degrees", 450.0, 1.0, 0.0),
            ("-30 degrees", -30.0, -0.5, sqrt3_over_2),
            ("-90 degrees", -90.0, -1.0, 0.0),
        ];

        for (name, deg, want_sin, want_cos) in cases {
            let angle = Angle::from_degrees(deg);
            let s = angle.sin();
            let c = angle.cos();
            let (sc_s, sc_c) = angle.sin_cos();

            assert!(
                approx_eq(s, want_sin),
                "Test case '{name}': sin({deg}) = {s}, want {want_sin}"
            );
            assert!(
                approx_eq(c, want_cos),
                "Test case '{name}': cos({deg}) = {c}, want {want_cos}"
            );
            assert!(
                approx_eq(sc_s, want_sin),
                "Test case '{name}': sin_cos({deg}).sin = {sc_s}, want {want_sin}"
            );
            assert!(
                approx_eq(sc_c, want_cos),
                "Test case '{name}': sin_cos({deg}).cos = {sc_c}, want {want_cos}"
            );
            assert!(
                approx_eq(s * s + c * c, 1.0),
                "Test case '{name}': Pythagorean identity failed"
            );
        }
    }

    #[test]
    fn test_constants() {
        assert!(approx_eq(
            Angle::ZERO.as_radians(),
            Angle::from_radians(0.0).as_radians()
        ));
        assert!(approx_eq(
            Angle::HALF_TURN.as_radians(),
            Angle::from_radians(-PI).as_radians()
        ));
        assert!(approx_eq(
            Angle::QUARTER_TURN.as_radians(),
            Angle::from_radians(FRAC_PI_2).as_radians()
        ));
    }

    #[test]
    fn test_degrees_and_radians_conversion() {
        use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

        let exact_cases = [
            ("zero", 0.0, 0.0),
            ("eighth_turn", 45.0, FRAC_PI_4),
            ("quarter_turn", 90.0, FRAC_PI_2),
            ("negative_quarter_turn", -90.0, -FRAC_PI_2),
            ("half_turn", 180.0, PI),
            ("three_quarter_turn", 270.0, 3.0 * FRAC_PI_2),
            ("full_turn", 360.0, TAU),
            ("negative_full_turn", -360.0, -TAU),
            ("one_and_half_turns", 540.0, 3.0 * PI),
        ];

        for (name, deg, rad) in exact_cases {
            let from_deg = Angle::from_degrees(deg);
            let from_rad = Angle::from_radians(rad);

            assert!(approx_eq(from_deg.as_degrees(), deg), "{name}: deg -> deg");
            assert!(approx_eq(from_deg.as_radians(), rad), "{name}: deg -> rad");
            assert!(approx_eq(from_rad.as_degrees(), deg), "{name}: rad -> deg");
            assert!(approx_eq(from_rad.as_radians(), rad), "{name}: rad -> rad");
        }

        let arbitrary_cases = [
            ("positive_fractional", 12.34),
            ("negative_fractional", -127.89),
            ("tiny_positive", 0.001),
            ("multiple_rotations_fractional", 1085.5),
        ];

        for (name, deg) in arbitrary_cases {
            let angle = Angle::from_degrees(deg);
            assert!(approx_eq(angle.as_degrees(), deg), "{name}: round-trip");
        }
    }

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    fn assert_angle_deg_eq(case: &str, got: Angle, want: Angle) {
        const DEG_EPSILON: f32 = 5e-5;

        let normalize_boundry = |degrees: f32| {
            if (degrees.abs() - 180.0).abs() < DEG_EPSILON {
                -180.0
            } else {
                degrees
            }
        };

        let got_deg = normalize_boundry(got.as_degrees());
        let want_deg = normalize_boundry(want.as_degrees());

        assert!(
            (got_deg - want_deg).abs() <= DEG_EPSILON,
            "Test case '{case}' failed: got {got_deg}, want {want_deg}"
        );
    }
}
