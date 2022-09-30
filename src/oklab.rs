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

use super::{clamp, max, min, LinearSrgb32};
use core::f32::consts::PI as PI_32;

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
    /// The distance along the `b` axis, form **sky blue** to **mustard yellow**.
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
        let l = max(0.0, lightness);
        let a = clamp(a, -0.5, 0.5);
        let b = clamp(b, -0.5, 0.5);

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
    pub fn squared_distance(&self, other: &Oklab32) -> f32 {
        (self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2)
    }

    ///
    /// - <https://www.w3.org/TR/css-color-4/#color-difference-OK>
    #[inline]
    pub fn distance(&self, other: &Oklab32) -> f32 {
        ((self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2))
            .sqrt()
    }
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
        let l = min(max(0.0, luminance), 100.0);
        let c = min(max(0.0, chroma), 0.5);
        let h = min(max(0.0, hue), 360.);

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

/// # Operations
impl Oklch32 {
    // TODO: use Oklab
    //
    // /// Measures the perceptual distance to another Oklch color
    // pub fn squared_distance(&self, other: &Oklch32) -> f32 {
    //     // CHECK:FIX: wrap polar coordinate
    //     (self.l - other.l).powi(2) + (self.c - other.c).powi(2) + (self.h - other.h).powi(2)
    // }
}

/* conversions */

/// Converts from [`Oklab32`] to [`Oklch32`] color spaces.
#[inline]
pub fn oklab_to_oklch_32(c: Oklab32) -> Oklch32 {
    // TODO CHECK both versions
    //
    // Oklch32 {
    //     l: c.l,
    //     c: c.a.hypot(c.b),
    //     h: c.b.atan2(c.a),
    // }

    let hue = c.b.atan2(c.a) * 180. / PI_32;
    #[rustfmt::skip]
    let h = if hue >= 0. { hue } else { hue + 360. };

    Oklch32 {
        l: c.l,
        c: c.a.hypot(c.b),
        h,
    }
    // function OKLab_to_OKLCH(OKLab) {
    //     var hue = Math.atan2(OKLab[2], OKLab[1]) * 180 / Math.PI;
    //     return [
    //         OKLab[0], // L is still L
    //         Math.sqrt(OKLab[1] ** 2 + OKLab[2] ** 2), // Chroma
    //         hue >= 0 ? hue : hue + 360 // Hue, in degrees [0 to 360)
    //     ];
    // }
}

/// Converts from [`Oklch32`] to [`Oklab32`] color spaces.
#[inline]
pub fn oklch_to_oklab_32(c: Oklch32) -> Oklab32 {
    // TODO: CHECK both versions
    // Oklab32 {
    //     l: c.l,
    //     a: c.c * c.h.cos(),
    //     b: c.c * c.h.sin(),
    // }

    Oklab32 {
        l: c.l,
        a: c.c * (c.h * PI_32 / 180.).cos(),
        b: c.c * (c.h * PI_32 / 180.).sin(),
    }
    // function OKLCH_to_OKLab(OKLCH) {
    //     return [
    //         OKLCH[0], // L is still L
    //         OKLCH[1] * Math.cos(OKLCH[2] * Math.PI / 180), // a
    //         OKLCH[1] * Math.sin(OKLCH[2] * Math.PI / 180)  // b
    //     ];
    // }
}

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

/*
// TODO

function XYZ_to_OKLab(XYZ) {
    // Given XYZ relative to D65, convert to OKLab
    var XYZtoLMS = [
        [ 0.8190224432164319,    0.3619062562801221,   -0.12887378261216414  ],
        [ 0.0329836671980271,    0.9292868468965546,     0.03614466816999844 ],
        [ 0.048177199566046255,  0.26423952494422764,    0.6335478258136937  ]
    ];
    var LMStoOKLab = [
        [  0.2104542553,   0.7936177850,  -0.0040720468 ],
        [  1.9779984951,  -2.4285922050,   0.4505937099 ],
        [  0.0259040371,   0.7827717662,  -0.8086757660 ]
    ];

    var LMS = multiplyMatrices(XYZtoLMS, XYZ);
    return multiplyMatrices(LMStoOKLab, LMS.map(c => Math.cbrt(c)));
    // L in range [0,1]. For use in CSS, multiply by 100 and add a percent
}

function OKLab_to_XYZ(OKLab) {
    // Given OKLab, convert to XYZ relative to D65
    var LMStoXYZ =  [
        [  1.2268798733741557,  -0.5578149965554813,   0.28139105017721583 ],
        [ -0.04057576262431372,  1.1122868293970594,  -0.07171106666151701 ],
        [ -0.07637294974672142, -0.4214933239627914,   1.5869240244272418  ]
    ];
    var OKLabtoLMS = [
        [ 0.99999999845051981432,  0.39633779217376785678,   0.21580375806075880339  ],
        [ 1.0000000088817607767,  -0.1055613423236563494,   -0.063854174771705903402 ],
        [ 1.0000000546724109177,  -0.089484182094965759684, -1.2914855378640917399   ]
    ];

    var LMSnl = multiplyMatrices(OKLabtoLMS, OKLab);
    return multiplyMatrices(LMStoXYZ, LMSnl.map(c => c ** 3));
}

*/

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

    impl From<Oklab32> for Oklch32 {
        #[inline]
        fn from(c: Oklab32) -> Oklch32 {
            oklab_to_oklch_32(c)
        }
    }
}
