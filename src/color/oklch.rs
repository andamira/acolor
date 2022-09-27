// acolor::color::lch
//
//! Oklch is a non-linear, perceptually uniform color space.
//!
//! It has the same `L` axis as [`Oklab`][super::oklab],
//! but uses polar coordinates C (Chroma) and H (Hue).
//!
//! # Links
//! - <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklch>
//

use super::Oklab32;

/// Oklch color reprsentation using `3` × [`f32`] components.
///
/// # Fields
/// - l: perceived luminosity
/// - c: chromacity
/// - h: hue
///
/// Best suited for perceptual color manipulation.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Oklch32 {
    /// Perceived lightness. A percentage between 0% and 100%.
    pub l: f32,
    /// Chromacity
    pub c: f32,
    /// Hue angle.
    /// - 0º points along the positive `a` axis (purplish red).
    /// - 90º points along the positive `b` axis (mustard yellow).
    /// - 180º points along the negative `a` axis (greenish cyan).
    /// - 90º points along the negative `b` axis (sky blue).
    pub h: f32,
}

/// # Constants
impl Oklch32 {
    ///
    pub const L_MAX: f32 = 100.;
    ///
    pub const L_MIN: f32 = 0.;

    /// theoretically unbounded (but in practice does not exceed 0.4).
    pub const C_MAX: f32 = 0.4;
    pub const C_MIN: f32 = 0.;

    pub const H_MAX: f32 = 360.;
    pub const H_MIN: f32 = 0.;
}

/// # Operations
impl Oklch32 {
    /// Measures the perceptual distance to another Oklch color
    pub fn squared_distance(&self, other: &Oklch32) -> f32 {
        // CHECK:FIX: wrap polar coordinate
        (self.l - other.l).powi(2) + (self.c - other.c).powi(2) + (self.h - other.h).powi(2)
    }
}

/* conversions */

/// Converts from [`Oklab32`] to [`Oklch32`] color spaces.
#[inline]
pub fn oklab_to_oklch_32(c: Oklab32) -> Oklch32 {
    Oklch32 {
        l: c.l,
        c: c.a.hypot(c.b),
        h: c.b.atan2(c.a),
    }
}

/// Converts from [`Oklch32`] to [`Oklab32`] color spaces.
#[inline]
pub fn oklch_to_oklab_32(c: Oklch32) -> Oklab32 {
    Oklab32 {
        l: c.l,
        a: c.c * c.h.cos(),
        b: c.c * c.h.sin(),
    }
}

mod impl_from {
    use super::*;

    impl From<Oklab32> for Oklch32 {
        #[inline]
        fn from(c: Oklab32) -> Oklch32 {
            oklab_to_oklch_32(c)
        }
    }
}

/// # Conversions
impl Oklch32 {
    /// Direct conversion from [`Oklab32`].
    #[inline]
    pub fn from_oklab(c: Oklab32) -> Oklch32 {
        c.into()
    }

    /// Direct conversion to [`Oklab32`].
    #[inline]
    pub fn to_oklab(&self) -> Oklab32 {
        oklch_to_oklab_32(*self)
    }
}
