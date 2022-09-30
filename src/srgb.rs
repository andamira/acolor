// acolor::srgb
//
//! Standard RGB color space.
//!
//! Linear and non-linear variants.
//!
//! # Links
//! - <https://en.wikipedia.org/wiki/SRGB>
//! - <https://www.w3.org/TR/css-color-4/#numeric-srgb>
//
// # TOC
// - definitions
//   - Srgb8
//   - Srgba8
//   - Srgb32
//   - Srgba32
//   - LinearSrgb32
//   - LinearSrgba32
// - conversions
// - utils
//

use iunorm::Unorm8;

use super::{linear_srgb_to_oklab_32, oklab_to_linear_srgb_32, Oklab32, GAMMA_32};

/* definitions */

/// Non-linear sRGB color representation using `3` × [`u8`] components.
///
/// Better suited for saving to the final graphics buffer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Srgb8 {
    /// Red luminosity.
    pub r: u8,
    /// Green luminosity.
    pub g: u8,
    /// Blue luminosity.
    pub b: u8,
}
/// # Constructors
impl Srgb8 {
    /// New Srgb8.
    pub fn new(r: u8, g: u8, b: u8) -> Srgb8 {
        Self { r, g, b }
    }
}

/// Non-linear sRGB+A color representation using `4` × [`u8`] components.
///
/// Better suited for saving to the final graphics buffer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Srgba8 {
    /// Red luminosity.
    pub r: u8,
    /// Green luminosity.
    pub g: u8,
    /// Blue luminosity.
    pub b: u8,
    /// Alpha
    pub a: u8,
}
/// # Constructors
impl Srgba8 {
    /// New Srgba8.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Srgba8 {
        Self { r, g, b, a }
    }
}

/// Non-linear sRGB color representation using `3` × [`f32`] components.
///
/// Values are normalized between `[0.0 .. 1.0]`
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Srgb32 {
    /// Red luminosity.
    pub r: f32,
    /// Green luminosity.
    pub g: f32,
    /// Blue luminosity.
    pub b: f32,
}
/// # Constructors
impl Srgb32 {
    /// New Srgb32.
    pub fn new(r: f32, g: f32, b: f32) -> Srgb32 {
        Self { r, g, b }
    }
}

/// Non-linear sRGB+A color representation using `4` × [`f32`] components.
///
/// Values are normalized between `[0.0 .. 1.0]`
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Srgba32 {
    /// Red luminosity.
    pub r: f32,
    /// Green luminosity.
    pub g: f32,
    /// Blue luminosity.
    pub b: f32,
    /// Alpha.
    pub a: f32,
}
/// # Constructors
impl Srgba32 {
    /// New Srgba32.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Srgba32 {
        Self { r, g, b, a }
    }
}

/// Linear sRGB color representation using `3` × [`f32`] components.
///
/// Values are normalized between `[0.0 .. 1.0]`
///
/// Better suited for physical calculations.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct LinearSrgb32 {
    /// Red luminosity.
    pub r: f32,
    /// Green luminosity.
    pub g: f32,
    /// Blue luminosity.
    pub b: f32,
}
/// # Constructors
impl LinearSrgb32 {
    /// New LinearSrgb32.
    pub fn new(r: f32, g: f32, b: f32) -> LinearSrgb32 {
        Self { r, g, b }
    }
}

/// Linear sRGB+A color representation using `4` × [`f32`] components.
///
/// Values are normalized between `[0.0 .. 1.0]`
///
/// Better suited for physical calculations.
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct LinearSrgba32 {
    /// Red luminosity.
    pub r: f32,
    /// Green luminosity.
    pub g: f32,
    /// Blue luminosity.
    pub b: f32,
    /// Alpha.
    pub a: f32,
}
/// # Constructors
impl LinearSrgba32 {
    /// New LinearSrgba32.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> LinearSrgba32 {
        Self { r, g, b, a }
    }
}

/* conversions */

/// Converts from [`LinearSrgb32`] to [`Srgb32`] color spaces.
#[inline]
pub fn linear_srgb_to_srgb_32(c: LinearSrgb32) -> Srgb32 {
    Srgb32 {
        r: nonlinearize_32(c.r, GAMMA_32),
        g: nonlinearize_32(c.g, GAMMA_32),
        b: nonlinearize_32(c.b, GAMMA_32),
    }
}

/// Converts from [`LinearSrgb32`] to [`Srgb32`] color spaces.
#[inline]
pub fn srgb_to_linear_srgb_32(c: Srgb32) -> LinearSrgb32 {
    LinearSrgb32 {
        r: linearize_32(c.r, GAMMA_32),
        g: linearize_32(c.g, GAMMA_32),
        b: linearize_32(c.b, GAMMA_32),
    }
}

/// # Direct conversions
impl Srgb8 {
    /// Direct conversion from [`Srgba8`].
    ///
    /// Looses the alpha channel.
    #[inline]
    pub fn from_srgba8(c: Srgba8) -> Srgb8 {
        Srgb8 {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }

    /// Direct conversion to [`Srgba8`].
    ///
    /// Expects the alpha channel.
    #[inline]
    pub fn to_srgba8(&self, alpha: u8) -> Srgba8 {
        Srgba8 {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    //

    /// Direct conversion from [`Srgb32`].
    #[inline]
    pub fn from_srgb32(c: Srgb32) -> Srgb8 {
        Self {
            r: Unorm8::from_f32(c.r).0,
            g: Unorm8::from_f32(c.g).0,
            b: Unorm8::from_f32(c.b).0,
        }
    }

    /// Direct conversion to [`Srgb32`].
    #[inline]
    pub fn to_srgb32(&self) -> Srgb32 {
        Srgb32 {
            r: Unorm8(self.r).to_f32(),
            g: Unorm8(self.g).to_f32(),
            b: Unorm8(self.b).to_f32(),
        }
    }
}

/// # Indirect conversions
impl Srgb8 {
    /// Indirect conversion from [`Oklab32`], through [`LinearSrgb32`] and [`Srgb32`].
    #[inline]
    pub fn from_oklab(c: Oklab32) -> Srgb8 {
        c.to_linear_srgb().to_srgb().into()
    }

    /// Indirect conversion to [`Oklab32`], through [`Srgb32`] and [`LinearSrgb32`].
    #[inline]
    pub fn to_oklab(&self) -> Oklab32 {
        self.to_srgb32().to_linear_srgb().to_oklab()
    }
}

/// # Direct conversions
impl Srgb32 {
    /// Direct conversion from [`Srgb8`].
    #[inline]
    pub fn from_srgb8(c: Srgb8) -> Srgb32 {
        c.into()
    }

    /// Direct conversion to [`Srgb8`].
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        Srgb8::from_srgb32(*self)
    }

    /// Direct conversion from [`LinearSrgb32`].
    #[inline]
    pub fn from_linear_srgb(c: LinearSrgb32) -> Srgb32 {
        c.into()
    }

    /// Direct conversion to [`LinearSrgb32`].
    #[inline]
    pub fn to_linear_srgb(&self) -> LinearSrgb32 {
        srgb_to_linear_srgb_32(*self)
    }
}

/// # Indirect conversions
impl Srgb32 {
    /// Indirect conversion from [`Oklab32`], through [`LinearSrgb32`].
    #[inline]
    pub fn from_oklab(c: Oklab32) -> Srgb32 {
        c.to_linear_srgb().into()
    }

    /// Indirect conversion to [`Oklab32`], through [`LinearSrgb32`].
    #[inline]
    pub fn to_oklab(&self) -> Oklab32 {
        self.to_linear_srgb().to_oklab()
    }
}

/// # Direct conversions
impl LinearSrgb32 {
    /// Direct conversion from [`Srgb32`].
    #[inline]
    pub fn from_srgb(c: Srgb32) -> LinearSrgb32 {
        c.into()
    }

    /// Direct conversion to [`Srgb32`].
    #[inline]
    pub fn to_srgb(&self) -> Srgb32 {
        linear_srgb_to_srgb_32(*self)
    }

    /// Direct conversion from [`Oklab32`].
    #[inline]
    pub fn from_oklab(c: Oklab32) -> LinearSrgb32 {
        c.into()
    }

    /// Direct conversion to [`Oklab32`].
    #[inline]
    pub fn to_oklab(&self) -> Oklab32 {
        linear_srgb_to_oklab_32(*self)
    }
}

/// # Indirect conversions
impl LinearSrgb32 {
    // TODO: OKLCH
}

mod impl_from {
    use super::*;

    /* Srgb8 */

    impl From<Srgb32> for Srgb8 {
        #[inline]
        fn from(c: Srgb32) -> Srgb8 {
            Srgb8::from_srgb32(c)
        }
    }
    impl From<Srgb8> for Srgb32 {
        #[inline]
        fn from(c: Srgb8) -> Srgb32 {
            Srgb8::to_srgb32(&c)
        }
    }
    /* LinearSrgb32 */

    impl From<LinearSrgb32> for Srgb32 {
        #[inline]
        fn from(c: LinearSrgb32) -> Srgb32 {
            linear_srgb_to_srgb_32(c)
        }
    }

    impl From<Srgb32> for LinearSrgb32 {
        #[inline]
        fn from(c: Srgb32) -> LinearSrgb32 {
            srgb_to_linear_srgb_32(c)
        }
    }

    /* Oklab32 */

    impl From<Oklab32> for LinearSrgb32 {
        #[inline]
        fn from(c: Oklab32) -> LinearSrgb32 {
            oklab_to_linear_srgb_32(c)
        }
    }
}

/* utils */

///
#[inline]
pub fn nonlinearize_32(linear: f32, gamma: f32) -> f32 {
    if linear >= 0.0031308 {
        (1.055) * linear.powf(1.0 / gamma) - 0.055
    } else {
        12.92 * linear
    }
}

///
#[inline]
pub fn linearize_32(nonlinear: f32, gamma: f32) -> f32 {
    if nonlinear >= 0.04045 {
        ((nonlinear + 0.055) / (1. + 0.055)).powf(gamma)
    } else {
        nonlinear / 12.92
    }
}

// TODO
// #[cfg(feature="half")]
// mod impl_half {
//     /// sRGB color space using 3 × [`u8`] components.
//     ///
//     /// Better suited for storage.
//     #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
//     pub struct Srgb16 {
//         pub r: f16,
//         pub g: f16,
//         pub b: f16,
//     }
//
//     /// Standard RGB linear color space with 4 × [`f16`] components.
//     ///
//     /// Better suited for normal operations.
//     #[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
//     pub struct LinearSrgb16 {
//         pub r: f16,
//         pub g: f16,
//         pub b: f16,
//     }
// }
