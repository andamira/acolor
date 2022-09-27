// acolor::color::oklab
//
//!
//! Oklab is a non-linear, perceptually uniform color space.
//!
//! Uses a D65 whitepoint.
//!
//! # Links
//! - <https://bottosson.github.io/posts/oklab/>
//! - <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklab>
//

use super::{oklab_to_oklch_32, oklch_to_oklab_32, LinearSrgb32, Oklch32};

/// Oklab color representation using `3` × [`f32`] components.
///
/// # Fields
/// - l: perceived luminosity
/// - a: green/red axis
/// - b: blue/yellow axis
///
/// Best suited for perceptual color manipulation.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Oklab32 {
    /// Perceived lightness. A percentage between 0% and 100%.
    pub l: f32,
    /// The distance along the `a` axis [greenish cyan]..[purplish red].
    pub a: f32,
    /// The distance along the `b` axis [sky blue]..[mustard yellow].
    pub b: f32,
}

impl Oklab32 {
    /// Measures the perceptual distance to another Oklch color
    // CHECK:FIX: saturate
    pub fn squared_distance(&self, other: &Oklab32) -> f32 {
        (self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2)
    }
}

/* conversions */

/// Converts from [`LinearSrgb32`] to [`Oklab32`] color spaces.
pub fn linear_srgb_to_oklab_32(c: LinearSrgb32) -> Oklab32 {
    let l = (0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b).cbrt();
    let m = (0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b).cbrt();
    let s = (0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b).cbrt();

    Oklab32 {
        l: 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        a: 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        b: 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    }
}

/// Converts from [`Oklab32`] to [`LinearSrgb32`] color spaces.
pub fn oklab_to_linear_srgb_32(c: Oklab32) -> LinearSrgb32 {
    let _l = c.l + 0.3963377774 * c.a + 0.2158037573 * c.b;
    let _m = c.l - 0.1055613458 * c.a - 0.0638541728 * c.b;
    let _s = c.l - 0.0894841775 * c.a - 1.2914855480 * c.b;

    let l = _l * _l * _l;
    let m = _m * _m * _m;
    let s = _s * _s * _s;

    LinearSrgb32 {
        r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    }
}

/// # Conversions
impl Oklab32 {
    /// Direct conversion from [`LinearSrgb32`].
    #[inline]
    pub fn from_linear_srgb(c: LinearSrgb32) -> Self {
        c.into()
    }

    /// Direct conversion to [`LinearSrgb32`].
    #[inline]
    pub fn to_linear_srgb(&self) -> LinearSrgb32 {
        oklab_to_linear_srgb_32(*self)
    }

    /// Direct conversion to [`Oklch32`].
    #[inline]
    pub fn to_oklch(&self) -> Oklch32 {
        oklab_to_oklch_32(*self)
    }

    /// Direct conversion from [`Oklch32`].
    #[inline]
    pub fn from_oklch(c: Oklch32) -> Self {
        c.into()
    }
}

mod impl_from {
    use super::*;

    impl From<LinearSrgb32> for Oklab32 {
        #[inline]
        fn from(c: LinearSrgb32) -> Oklab32 {
            linear_srgb_to_oklab_32(c)
        }
    }

    impl From<Oklch32> for Oklab32 {
        #[inline]
        fn from(c: Oklch32) -> Oklab32 {
            oklch_to_oklab_32(c)
        }
    }
}
