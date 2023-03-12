// acolor::oklab
//
//! Oklab is a non-linear, perceptually uniform color space.
//!
//! Oklch is the corresponding polar form of Oklab.
//!
//! Uses a D65 whitepoint.
//!
//! # Links
//! - <https://bottosson.github.io/posts/oklab/>
//! - <https://www.w3.org/TR/css-color-4/#ok-lab>
//! - <https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklab>
//

use super::{LinearSrgb32, LinearSrgba32, Srgb32, Srgb8, Srgba32, Srgba8};
use devela::{pclamp, pmax};

#[cfg(all(feature = "no-std", not(feature = "std")))]
use libm::{atan2f, cbrtf, cosf, hypotf, powf, sinf};

/* definitions */

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
    /// The distance along the `a` axis from **greenish cyan** to **purplish red**.
    pub a: f32,
    /// The distance along the `b` axis, from **sky blue** to **mustard yellow**.
    pub b: f32,
}

/// # Constructors
impl Oklab32 {
    /// New Oklab color.
    ///
    /// # Arguments
    /// - **lighness**, tipically between `0.` and `100.`, range: `> 0.`.
    /// - **a**, cyan..red axis, range: `-0.5..0.5`.
    /// - **b**, blue..yellow axis, range: `-0.5..0.5`.
    pub fn new(lightness: f32, a: f32, b: f32) -> Oklab32 {
        let l = pmax(0.0, lightness);
        let a = pclamp(a, -0.5, 0.5);
        let b = pclamp(b, -0.5, 0.5);

        Self { l, a, b }
    }
}

/// # Constants
impl Oklab32 {
    ///
    pub const L_MIN: f32 = 0.;
    ///
    pub const L_MAX: f32 = 100.;

    ///
    pub const A_MIN: f32 = -0.5;
    ///
    pub const A_MAX: f32 = 0.5;

    ///
    pub const B_MIN: f32 = -0.5;
    ///
    pub const B_MAX: f32 = 0.5;
}

/// # Operations
impl Oklab32 {
    /// Measures the perceptual distance to another Oklab color
    // CHECK:FIX: saturate
    #[inline]
    #[cfg(any(feature = "std", feature = "no-std"))]
    #[cfg_attr(
        feature = "nightly",
        doc(cfg(any(feature = "std", feature = "no-std")))
    )]
    pub fn squared_distance(&self, other: &Oklab32) -> f32 {
        #[cfg(feature = "std")]
        return (self.l - other.l).powi(2)
            + (self.a - other.a).powi(2)
            + (self.b - other.b).powi(2);

        #[cfg(not(feature = "std"))]
        return powf(self.l - other.l, 2.)
            + powf(self.a - other.a, 2.)
            + powf(self.b - other.b, 2.);
    }

    // ///
    // /// - <https://www.w3.org/TR/css-color-4/#color-difference-OK>
    // #[inline]
    // pub fn distance(&self, other: &Oklab32) -> f32 {
    //     ((self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2))
    //         .sqrt()
    // }
}

/// Oklch color representation using `3` × [`f32`] components.
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

/// # Constructors
impl Oklch32 {
    /// New Oklch color with clamped values.
    pub fn new(luminance: f32, chroma: f32, hue: f32) -> Oklch32 {
        let l = pclamp(luminance, 0.0, 100.0);
        let c = pclamp(chroma, 0.0, 0.5);
        let h = pclamp(hue, 0.0, 360.);

        Self { l, c, h }
    }
}

/// # Constants
impl Oklch32 {
    /// Luminance minimum value.
    pub const L_MIN: f32 = 0.;
    /// Luminance maximum value.
    pub const L_MAX: f32 = 100.;

    /// Chroma minimum value.
    pub const C_MIN: f32 = 0.;
    /// Chroma maximum value.
    pub const C_MAX: f32 = 0.5;

    /// Hue minimum value.
    pub const H_MIN: f32 = 0.;
    /// Hue maximum value.
    pub const H_MAX: f32 = 360.;
}

// // TODO
// /// # Operations
// impl Oklch32 {
// }

/* conversions */

// Converts from [`Oklab32`] to [`Oklch32`] color spaces.
#[inline]
#[cfg(any(feature = "std", feature = "no-std"))]
fn oklab32_to_oklch32(c: Oklab32) -> Oklch32 {
    #[cfg(feature = "std")]
    {
        use core::f32::consts::PI as PI_32;
        let hue = c.b.atan2(c.a) * 180. / PI_32;
        #[rustfmt::skip]
        let h = if hue >= 0. { hue } else { hue + 360. };

        Oklch32 {
            l: c.l,
            c: c.a.hypot(c.b),
            h,
        }
    }

    #[cfg(not(feature = "std"))]
    {
        use core::f32::consts::PI as PI_32;
        let hue = atan2f(c.b, c.a) * 180. / PI_32;
        #[rustfmt::skip]
        let h = if hue >= 0. { hue } else { hue + 360. };

        Oklch32 {
            l: c.l,
            c: hypotf(c.a, c.b),
            h,
        }
    }
}

// Converts from [`Oklch32`] to [`Oklab32`] color spaces.
#[inline]
#[cfg(any(feature = "std", feature = "no-std"))]
fn oklch32_to_oklab32(c: Oklch32) -> Oklab32 {
    #[cfg(feature = "std")]
    {
        use core::f32::consts::PI as PI_32;
        Oklab32 {
            l: c.l,
            a: c.c * (c.h * PI_32 / 180.).cos(),
            b: c.c * (c.h * PI_32 / 180.).sin(),
        }
    }

    #[cfg(not(feature = "std"))]
    {
        use core::f32::consts::PI as PI_32;
        Oklab32 {
            l: c.l,
            a: c.c * cosf(c.h * PI_32 / 180.),
            b: c.c * sinf(c.h * PI_32 / 180.),
        }
    }
}

/// Converts from [`LinearSrgb32`] to [`Oklab32`] color spaces.
#[cfg(any(feature = "std", feature = "no-std"))]
fn linear_srgb32_to_oklab32(c: LinearSrgb32) -> Oklab32 {
    #[cfg(feature = "std")]
    let l = (0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b).cbrt();
    #[cfg(not(feature = "std"))]
    let l = cbrtf(0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b);
    #[cfg(feature = "std")]
    let m = (0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b).cbrt();
    #[cfg(not(feature = "std"))]
    let m = cbrtf(0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b);
    #[cfg(feature = "std")]
    let s = (0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b).cbrt();
    #[cfg(not(feature = "std"))]
    let s = cbrtf(0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b);

    Oklab32 {
        l: 0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        a: 1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        b: 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    }
}

/// Converts from [`Oklab32`] to [`LinearSrgb32`] color spaces.
fn oklab32_to_linear_srgb32(c: Oklab32) -> LinearSrgb32 {
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

/// # Direct conversions
impl Oklab32 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub fn from_array(c: [f32; 3]) -> Oklab32 {
        Oklab32 {
            l: c[0],
            a: c[1],
            b: c[2],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub fn to_array(c: Oklab32) -> [f32; 3] {
        [c.l, c.a, c.b]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub fn from_tuple(c: (f32, f32, f32)) -> Oklab32 {
        Oklab32 {
            l: c.0,
            a: c.1,
            b: c.2,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub fn to_tuple(c: Oklab32) -> (f32, f32, f32) {
        (c.l, c.a, c.b)
    }
}

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Oklab32 {
    // LinearSrgb32

    /// Direct conversion from [`LinearSrgb32`].
    #[inline]
    pub fn from_linear_srgb32(c: LinearSrgb32) -> Oklab32 {
        linear_srgb32_to_oklab32(c)
    }

    /// Direct conversion to [`LinearSrgb32`].
    #[inline]
    pub fn to_linear_srgb32(&self) -> LinearSrgb32 {
        oklab32_to_linear_srgb32(*self)
    }

    // LinearSrgba32

    /// Direct conversion from [`LinearSrgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_linear_srgba32(c: LinearSrgba32) -> Oklab32 {
        c.to_linear_srgb32().to_oklab32()
    }

    /// Direct conversion to [`LinearSrgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_linear_srgba32(&self, alpha: f32) -> LinearSrgba32 {
        oklab32_to_linear_srgb32(*self).to_linear_srgba32(alpha)
    }

    // Oklch32

    /// Direct conversion to [`Oklch32`].
    #[inline]
    pub fn to_oklch32(&self) -> Oklch32 {
        oklab32_to_oklch32(*self)
    }

    /// Direct conversion from [`Oklch32`].
    #[inline]
    pub fn from_oklch32(c: Oklch32) -> Oklab32 {
        oklch32_to_oklab32(c)
    }
}

/// # Indirect conversions
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Oklab32 {
    // Srgb8

    /// Indirect conversion from [`Srgb8`].
    #[inline]
    pub fn from_srgb8(c: Srgb8) -> Oklab32 {
        c.to_oklab32()
    }

    /// Indirect conversion to [`Srgb8`].
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        self.to_linear_srgb32().to_srgb32().to_srgb8()
    }

    // Srgb8

    /// Indirect conversion from [`Srgba8`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_srgba8(c: Srgba8) -> Oklab32 {
        c.to_oklab32()
    }

    /// Indirect conversion to [`Srgba8`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba8(&self, alpha: u8) -> Srgba8 {
        self.to_linear_srgb32().to_srgb32().to_srgba8(alpha)
    }

    // Srgb32

    /// Indirect conversion from [`Srgb32`].
    #[inline]
    pub fn from_srgb32(c: Srgb32) -> Oklab32 {
        c.to_oklab32()
    }

    /// Indirect conversion to [`Srgb32`].
    #[inline]
    pub fn to_srgb32(&self) -> Srgb32 {
        self.to_linear_srgb32().to_srgb32()
    }

    // Srgba32

    /// Indirect conversion from [`Srgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_srgba32(c: Srgba32) -> Oklab32 {
        c.to_oklab32()
    }

    /// Indirect conversion to [`Srgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba32(&self, alpha: f32) -> Srgba32 {
        self.to_linear_srgb32().to_srgba32(alpha)
    }
}

/// # Direct conversions
impl Oklch32 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub fn from_array(c: [f32; 3]) -> Oklch32 {
        Oklch32 {
            l: c[0],
            c: c[1],
            h: c[2],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub fn to_array(c: Oklch32) -> [f32; 3] {
        [c.l, c.c, c.h]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub fn from_tuple(c: (f32, f32, f32)) -> Oklch32 {
        Oklch32 {
            l: c.0,
            c: c.1,
            h: c.2,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub fn to_tuple(c: Oklch32) -> (f32, f32, f32) {
        (c.l, c.c, c.h)
    }
}

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Oklch32 {
    // Oklab32

    /// Direct conversion from [`Oklab32`].
    #[inline]
    pub fn from_oklab32(c: Oklab32) -> Oklch32 {
        oklab32_to_oklch32(c)
    }

    /// Direct conversion to [`Oklab32`].
    #[inline]
    pub fn to_oklab32(&self) -> Oklab32 {
        oklch32_to_oklab32(*self)
    }

    // Srgb8

    /// Indirect conversion from [`Srgb8`].
    #[inline]
    pub fn from_srgb8(c: Srgb8) -> Oklch32 {
        c.to_oklch32()
    }

    /// Indirect conversion to [`Srgb8`].
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        self.to_oklab32().to_linear_srgb32().to_srgb32().to_srgb8()
    }

    // Srgb8

    /// Indirect conversion from [`Srgba8`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_srgba8(c: Srgba8) -> Oklch32 {
        c.to_oklch32()
    }

    /// Indirect conversion to [`Srgba8`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba8(&self, alpha: u8) -> Srgba8 {
        self.to_oklab32()
            .to_linear_srgb32()
            .to_srgb32()
            .to_srgba8(alpha)
    }

    // Srgb32

    /// Indirect conversion from [`Srgb32`].
    #[inline]
    pub fn from_srgb32(c: Srgb32) -> Oklch32 {
        c.to_oklch32()
    }
    /// Indirect conversion to [`Srgb32`].
    #[inline]
    pub fn to_srgb32(&self) -> Srgb32 {
        self.to_oklab32().to_linear_srgb32().to_srgb32()
    }

    // Srgba32

    /// Indirect conversion from [`Srgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_srgba32(c: Srgba32) -> Oklch32 {
        c.to_oklch32()
    }

    /// Indirect conversion to [`Srgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba32(&self, alpha: f32) -> Srgba32 {
        self.to_oklab32().to_linear_srgb32().to_srgba32(alpha)
    }

    // LinearSrgb32

    /// Indirect conversion from [`LinearSrgb32`].
    #[inline]
    pub fn from_linear_srgb32(c: LinearSrgb32) -> Oklch32 {
        c.to_oklch32()
    }

    /// Indirect conversion to [`LinearSrgba32`].
    #[inline]
    pub fn to_linear_srgb32(&self) -> LinearSrgb32 {
        self.to_oklab32().to_linear_srgb32()
    }

    // LinearSrgba32

    /// Indirect conversion from [`LinearSrgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_linear_srgba32(c: LinearSrgba32) -> Oklch32 {
        c.to_oklch32()
    }

    /// Indirect conversion to [`LinearSrgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_linear_srgba32(&self, alpha: f32) -> LinearSrgba32 {
        self.to_oklab32().to_linear_srgba32(alpha)
    }
}

mod impl_from {
    use super::*;

    /* From/Into arrays and tuples */

    impl From<[f32; 3]> for Oklab32 {
        #[inline]
        fn from(c: [f32; 3]) -> Oklab32 {
            Oklab32::new(c[0], c[1], c[2])
        }
    }

    impl From<(f32, f32, f32)> for Oklab32 {
        #[inline]
        fn from(c: (f32, f32, f32)) -> Oklab32 {
            Oklab32::new(c.0, c.1, c.2)
        }
    }

    impl From<[f32; 3]> for Oklch32 {
        #[inline]
        fn from(c: [f32; 3]) -> Oklch32 {
            Oklch32::new(c[0], c[1], c[2])
        }
    }

    impl From<(f32, f32, f32)> for Oklch32 {
        #[inline]
        fn from(c: (f32, f32, f32)) -> Oklch32 {
            Oklch32::new(c.0, c.1, c.2)
        }
    }

    /* From Oklab32 */

    impl From<Oklab32> for [f32; 3] {
        #[inline]
        fn from(c: Oklab32) -> [f32; 3] {
            [c.l, c.a, c.b]
        }
    }

    impl From<Oklab32> for (f32, f32, f32) {
        #[inline]
        fn from(c: Oklab32) -> (f32, f32, f32) {
            (c.l, c.a, c.b)
        }
    }
}

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
mod impl_from_std {
    use super::*;

    impl From<Oklab32> for Oklch32 {
        #[inline]
        fn from(c: Oklab32) -> Oklch32 {
            c.to_oklch32()
        }
    }

    impl From<Oklab32> for Srgb8 {
        #[inline]
        fn from(c: Oklab32) -> Srgb8 {
            c.to_linear_srgb32().to_srgb32().to_srgb8()
        }
    }

    impl From<Oklab32> for Srgba8 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Oklab32) -> Srgba8 {
            c.to_srgba8(u8::MAX)
        }
    }

    impl From<Oklab32> for Srgb32 {
        #[inline]
        fn from(c: Oklab32) -> Srgb32 {
            c.to_srgb32()
        }
    }

    impl From<Oklab32> for Srgba32 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Oklab32) -> Srgba32 {
            c.to_srgba32(1.)
        }
    }

    impl From<Oklab32> for LinearSrgb32 {
        #[inline]
        fn from(c: Oklab32) -> LinearSrgb32 {
            c.to_linear_srgb32()
        }
    }

    impl From<Oklab32> for LinearSrgba32 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Oklab32) -> LinearSrgba32 {
            c.to_linear_srgba32(1.)
        }
    }

    /* From Oklch32 */

    impl From<Oklch32> for [f32; 3] {
        #[inline]
        fn from(c: Oklch32) -> [f32; 3] {
            [c.l, c.c, c.h]
        }
    }

    impl From<Oklch32> for (f32, f32, f32) {
        #[inline]
        fn from(c: Oklch32) -> (f32, f32, f32) {
            (c.l, c.c, c.h)
        }
    }

    impl From<Oklch32> for Oklab32 {
        #[inline]
        fn from(c: Oklch32) -> Oklab32 {
            c.to_oklab32()
        }
    }
}
