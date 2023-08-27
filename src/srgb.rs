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
//
// - definitions & constructors:
//   - Srgb8
//   - Srgba8
//   - Srgb32
//   - Srgba32
//   - LinearSrgb32
//   - LinearSrgba32
// - conversions:
//   - Srgb8
//   - Srgba8
//   - Srgb32
//   - Srgba32
//   - LinearSrgb32
//   - LinearSrgba32
// - utils
//   - linearize32
//   - nonlinearize32
//

#[cfg(any(feature = "std", feature = "no-std"))]
use crate::{
    oklab::{Oklab32, Oklch32},
    GAMMA_32,
};
use iunorm::Unorm8;

// DEFINITIONS
// -----------------------------------------------------------------------------

/// Non-linear sRGB color representation using `3` × [`u8`] components.
///
/// Better suited for saving to the final graphics buffer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Srgb8 {
    /// Gamma encoded red luminosity.
    pub r: u8,
    /// Gamma encoded green luminosity.
    pub g: u8,
    /// Gamma encoded blue luminosity.
    pub b: u8,
}
/// # Constructors
impl Srgb8 {
    /// New Srgb8.
    pub const fn new(r: u8, g: u8, b: u8) -> Srgb8 {
        Self { r, g, b }
    }
}

/// Non-linear sRGB+A color representation using `4` × [`u8`] components.
///
/// Better suited for saving to the final graphics buffer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Srgba8 {
    /// Gamma encoded red luminosity.
    pub r: u8,
    /// Gamma encoded green luminosity.
    pub g: u8,
    /// Gamma encoded blue luminosity.
    pub b: u8,
    /// Linear alpha channel.
    pub a: u8,
}
/// # Constructors
impl Srgba8 {
    /// New Srgba8.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Srgba8 {
        Self { r, g, b, a }
    }
}

/// Non-linear sRGB color representation using `3` × [`f32`] components.
///
/// Values are normalized between `[0.0 .. 1.0]`
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Srgb32 {
    /// Gamma encoded red luminosity.
    pub r: f32,
    /// Gamma encoded green luminosity.
    pub g: f32,
    /// Gamma encoded blue luminosity.
    pub b: f32,
}
/// # Constructors
impl Srgb32 {
    /// New Srgb32.
    pub const fn new(r: f32, g: f32, b: f32) -> Srgb32 {
        Self { r, g, b }
    }
}

/// Non-linear sRGB+A color representation using `4` × [`f32`] components.
///
/// Values are normalized between `[0.0 .. 1.0]`
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Srgba32 {
    /// Gamma encoded red luminosity.
    pub r: f32,
    /// Gamma encoded green luminosity.
    pub g: f32,
    /// Gamma encoded blue luminosity.
    pub b: f32,
    /// Linear alpha channel.
    pub a: f32,
}
/// # Constructors
impl Srgba32 {
    /// New Srgba32.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Srgba32 {
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
    /// Linear red luminosity.
    pub r: f32,
    /// Linear green luminosity.
    pub g: f32,
    /// Linear blue luminosity.
    pub b: f32,
}
/// # Constructors
impl LinearSrgb32 {
    /// New LinearSrgb32.
    pub const fn new(r: f32, g: f32, b: f32) -> LinearSrgb32 {
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
    /// Linear red luminosity.
    pub r: f32,
    /// Linear green luminosity.
    pub g: f32,
    /// Linear blue luminosity.
    pub b: f32,
    /// Linear alpha channel.
    pub a: f32,
}
/// # Constructors
impl LinearSrgba32 {
    /// New LinearSrgba32.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> LinearSrgba32 {
        Self { r, g, b, a }
    }
}

// CONVERSIONS
// -----------------------------------------------------------------------------

/* conversions: Srgb8 */

/// # Direct conversions
impl Srgb8 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub const fn from_array(c: [u8; 3]) -> Srgb8 {
        Srgb8 {
            r: c[0],
            g: c[1],
            b: c[2],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub const fn to_array(c: Srgb8) -> [u8; 3] {
        [c.r, c.g, c.b]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub const fn from_tuple(c: (u8, u8, u8)) -> Srgb8 {
        Srgb8 {
            r: c.0,
            g: c.1,
            b: c.2,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub const fn to_tuple(c: Srgb8) -> (u8, u8, u8) {
        (c.r, c.g, c.b)
    }

    // Srgba8

    /// Direct conversion from [`Srgba8`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub const fn from_srgba8(c: Srgba8) -> Srgb8 {
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
    pub const fn to_srgba8(&self, alpha: u8) -> Srgba8 {
        Srgba8 {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    // Srgb32

    /// Direct conversion from [`Srgb32`].
    #[inline]
    pub fn from_srgb32(c: Srgb32) -> Srgb8 {
        Srgb8 {
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

    // Srgba32

    /// Direct conversion from [`Srgba32`].
    #[inline]
    pub fn from_srgba32(c: Srgba32) -> Srgb8 {
        Srgb8 {
            r: Unorm8::from_f32(c.r).0,
            g: Unorm8::from_f32(c.g).0,
            b: Unorm8::from_f32(c.b).0,
        }
    }

    /// Direct conversion to [`Srgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba32(&self, alpha: f32) -> Srgba32 {
        Srgba32 {
            r: Unorm8(self.r).to_f32(),
            g: Unorm8(self.g).to_f32(),
            b: Unorm8(self.b).to_f32(),
            a: alpha,
        }
    }
}

/// # Indirect conversions
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Srgb8 {
    // LinearSrgb32

    /// Indirect conversion from [`LinearSrgb32`].
    #[inline]
    pub fn from_linear_srgb32(c: LinearSrgb32) -> Srgb8 {
        c.to_srgb32().to_srgb8()
    }

    /// Indirect conversion to [`LinearSrgb32`].
    #[inline]
    pub fn to_linear_srgb32(&self) -> LinearSrgb32 {
        self.to_srgb32().to_linear_srgb32()
    }

    // LinearSrgba32

    /// Indirect conversion from [`LinearSrgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_linear_srgba32(c: LinearSrgba32) -> Srgb8 {
        c.to_srgb32().to_srgb8()
    }

    /// Indirect conversion to [`LinearSrgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_linear_srgba32(&self, alpha: f32) -> LinearSrgba32 {
        self.to_srgb32().to_linear_srgba32(alpha)
    }

    // Oklab32

    /// Indirect conversion from [`Oklab32`].
    #[inline]
    pub fn from_oklab32(c: Oklab32) -> Srgb8 {
        c.to_linear_srgb32().to_srgb32().to_srgb8()
    }

    /// Indirect conversion to [`Oklab32`].
    #[inline]
    pub fn to_oklab32(&self) -> Oklab32 {
        self.to_srgb32().to_linear_srgb32().to_oklab32()
    }

    // Oklch32

    /// Indirect conversion from [`Oklch32`].
    #[inline]
    pub fn from_oklch32(c: Oklch32) -> Srgb8 {
        c.to_oklab32().to_linear_srgb32().to_srgb32().to_srgb8()
    }

    /// Indirect conversion to [`Oklch32`].
    #[inline]
    pub fn to_oklch32(&self) -> Oklch32 {
        self.to_oklab32().to_oklch32()
    }
}

/* conversions: Srgba8 */

/// # Direct conversions
impl Srgba8 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub fn from_array(c: [u8; 4]) -> Srgba8 {
        Srgba8 {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub fn to_array(c: Srgba8) -> [u8; 4] {
        [c.r, c.g, c.b, c.a]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub fn from_tuple(c: (u8, u8, u8, u8)) -> Srgba8 {
        Srgba8 {
            r: c.0,
            g: c.1,
            b: c.2,
            a: c.3,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub fn to_tuple(c: Srgba8) -> (u8, u8, u8, u8) {
        (c.r, c.g, c.b, c.a)
    }

    // Srgb8

    /// Direct conversion from [`Srgb8`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_srgb8(c: Srgb8, alpha: u8) -> Srgba8 {
        Srgba8 {
            r: c.r,
            g: c.g,
            b: c.b,
            a: alpha,
        }
    }
    /// Direct conversion to [`Srgb8`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        Srgb8 {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    // Srgb32

    /// Direct conversion from [`Srgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_srgb32(c: Srgb32, alpha: u8) -> Srgba8 {
        Srgba8 {
            r: Unorm8::from_f32(c.r).0,
            g: Unorm8::from_f32(c.g).0,
            b: Unorm8::from_f32(c.b).0,
            a: alpha,
        }
    }

    /// Direct conversion to [`Srgb32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_srgb32(&self) -> Srgb32 {
        Srgb32 {
            r: Unorm8(self.r).to_f32(),
            g: Unorm8(self.g).to_f32(),
            b: Unorm8(self.b).to_f32(),
        }
    }

    // Srgba32

    /// Direct conversion from [`Srgba32`].
    #[inline]
    pub fn from_srgba32(c: Srgba32) -> Srgba8 {
        Srgba8 {
            r: Unorm8::from_f32(c.r).0,
            g: Unorm8::from_f32(c.g).0,
            b: Unorm8::from_f32(c.b).0,
            a: Unorm8::from_f32(c.a).0,
        }
    }

    /// Direct conversion to [`Srgba32`].
    #[inline]
    pub fn to_srgba32(&self) -> Srgba32 {
        Srgba32 {
            r: Unorm8(self.r).to_f32(),
            g: Unorm8(self.g).to_f32(),
            b: Unorm8(self.b).to_f32(),
            a: Unorm8(self.a).to_f32(),
        }
    }
}

/// # Indirect conversions
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Srgba8 {
    // LinearSrgb32

    /// Indirect conversion from [`Srgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_linear_srgb32(c: LinearSrgb32, alpha: u8) -> Srgba8 {
        c.to_srgb32().to_srgba8(alpha)
    }

    /// Indirect conversion to [`LinearSrgb32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_linear_srgb32(&self) -> LinearSrgb32 {
        self.to_srgb32().to_linear_srgb32()
    }

    // LinearSrgba32

    /// Indirect conversion from [`LinearSrgba32`].
    #[inline]
    pub fn from_linear_srgba32(c: LinearSrgba32) -> Srgba8 {
        c.to_srgba32().to_srgba8()
    }

    /// Indirect conversion to [`LinearSrgba32`].
    #[inline]
    pub fn to_linear_srgba32(&self) -> LinearSrgba32 {
        self.to_srgba32().to_linear_srgba32()
    }

    // Oklab

    /// Indirect conversion from [`Oklab32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_oklab32(c: Oklab32, alpha: u8) -> Srgba8 {
        c.to_linear_srgb32().to_srgb32().to_srgba8(alpha)
    }

    /// Indirect conversion to [`Oklab32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_oklab32(&self) -> Oklab32 {
        self.to_srgb32().to_linear_srgb32().to_oklab32()
    }

    // Oklch32

    /// Indirect conversion from [`Oklch32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_oklch32(c: Oklch32, alpha: u8) -> Srgba8 {
        c.to_oklab32()
            .to_linear_srgb32()
            .to_srgb32()
            .to_srgba8(alpha)
    }

    /// Indirect conversion to [`Oklch32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_oklch32(&self) -> Oklch32 {
        self.to_oklab32().to_oklch32()
    }
}

/* conversions: Srgb32 */

/// # Direct conversions
impl Srgb32 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub fn from_array(c: [f32; 3]) -> Srgb32 {
        Srgb32 {
            r: c[0],
            g: c[1],
            b: c[2],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub fn to_array(c: Srgb32) -> [f32; 3] {
        [c.r, c.g, c.b]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub fn from_tuple(c: (f32, f32, f32)) -> Srgb32 {
        Srgb32 {
            r: c.0,
            g: c.1,
            b: c.2,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub fn to_tuple(c: Srgb32) -> (f32, f32, f32) {
        (c.r, c.g, c.b)
    }

    // Srgb8

    /// Direct conversion from [`Srgb8`].
    #[inline]
    pub fn from_srgb8(c: Srgb8) -> Srgb32 {
        c.to_srgb32()
    }
    /// Direct conversion to [`Srgb8`].
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        Srgb8::from_srgb32(*self)
    }

    // Srgba8

    /// Direct conversion from [`Srgba8`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_srgba8(c: Srgba8) -> Srgb32 {
        c.to_srgb32()
    }
    /// Direct conversion to [`Srgba8`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba8(&self, alpha: u8) -> Srgba8 {
        Srgba8::from_srgb32(*self, alpha)
    }

    // Srgba32

    /// Direct conversion from [`Srgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub const fn from_srgba32(c: Srgba32) -> Srgb32 {
        Srgb32::new(c.r, c.g, c.b)
    }
    /// Direct conversion to [`Srgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub const fn to_srgba32(&self, alpha: f32) -> Srgba32 {
        Srgba32::new(self.r, self.g, self.b, alpha)
    }
}

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Srgb32 {
    // LinearSrgb32

    /// Direct conversion from [`LinearSrgb32`].
    #[inline]
    pub fn from_linear_srgb32(c: LinearSrgb32) -> Srgb32 {
        Srgb32 {
            r: nonlinearize32(c.r, GAMMA_32),
            g: nonlinearize32(c.g, GAMMA_32),
            b: nonlinearize32(c.b, GAMMA_32),
        }
    }
    /// Direct conversion to [`LinearSrgb32`].
    #[inline]
    pub fn to_linear_srgb32(&self) -> LinearSrgb32 {
        LinearSrgb32 {
            r: linearize32(self.r, GAMMA_32),
            g: linearize32(self.g, GAMMA_32),
            b: linearize32(self.b, GAMMA_32),
        }
    }

    // LinearSrgba32

    /// Direct conversion from [`LinearSrgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_linear_srgba32(c: LinearSrgba32) -> Srgb32 {
        Srgb32 {
            r: nonlinearize32(c.r, GAMMA_32),
            g: nonlinearize32(c.g, GAMMA_32),
            b: nonlinearize32(c.b, GAMMA_32),
        }
    }
    /// Direct conversion to [`LinearSrgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_linear_srgba32(&self, alpha: f32) -> LinearSrgba32 {
        LinearSrgba32 {
            r: linearize32(self.r, GAMMA_32),
            g: linearize32(self.g, GAMMA_32),
            b: linearize32(self.b, GAMMA_32),
            a: alpha,
        }
    }
}

/// # Indirect conversions
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Srgb32 {
    // Oklab32

    /// Indirect conversion from [`Oklab32`], through [`LinearSrgb32`].
    #[inline]
    pub fn from_oklab32(c: Oklab32) -> Srgb32 {
        c.to_linear_srgb32().to_srgb32()
    }

    /// Indirect conversion to [`Oklab32`], through [`LinearSrgb32`].
    #[inline]
    pub fn to_oklab32(&self) -> Oklab32 {
        self.to_linear_srgb32().to_oklab32()
    }

    // Oklch32

    /// Indirect conversion from [`Oklch32`].
    #[inline]
    pub fn from_oklch32(c: Oklch32) -> Srgb32 {
        c.to_oklab32().to_linear_srgb32().to_srgb32()
    }

    /// Indirect conversion to [`Oklch32`].
    #[inline]
    pub fn to_oklch32(&self) -> Oklch32 {
        self.to_oklab32().to_oklch32()
    }
}

/* conversions: Srgba32 */

/// # Direct conversions
///
/// - From/Into [`Srgb8`], [`Srgba8`], [`Srgb32`], [`LinearSrgb32`], [`LinearSrgba32`].
impl Srgba32 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub const fn from_array(c: [f32; 4]) -> Srgba32 {
        Srgba32 {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub const fn to_array(c: Srgba32) -> [f32; 4] {
        [c.r, c.g, c.b, c.a]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub const fn from_tuple(c: (f32, f32, f32, f32)) -> Srgba32 {
        Srgba32 {
            r: c.0,
            g: c.1,
            b: c.2,
            a: c.3,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub const fn to_tuple(c: Srgba32) -> (f32, f32, f32, f32) {
        (c.r, c.g, c.b, c.a)
    }

    // Srgb8

    /// Direct conversion from [`Srgb8`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_srgb8(c: Srgb8, alpha: f32) -> Srgba32 {
        c.to_srgba32(alpha)
    }
    /// Direct conversion to [`Srgb8`].
    ///
    /// Removes the alpha channel.
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        Srgb8::from_srgba32(*self)
    }

    // Srgba8

    /// Direct conversion from [`Srgba8`].
    #[inline]
    pub fn from_srgba8(c: Srgba8) -> Srgba32 {
        c.to_srgba32()
    }
    /// Direct conversion to [`Srgba8`].
    #[inline]
    pub fn to_srgba8(&self) -> Srgba8 {
        Srgba8::from_srgba32(*self)
    }

    // Srgb32

    /// Direct conversion from [`Srgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_srgb32(c: Srgb32, alpha: f32) -> Srgba32 {
        c.to_srgba32(alpha)
    }
    /// Direct conversion to [`Srgb32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub const fn to_srgb32(&self) -> Srgb32 {
        Srgb32::from_srgba32(*self)
    }
}

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Srgba32 {
    // LinearSrgb32

    /// Direct conversion from [`LinearSrgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_linear_srgb32(c: LinearSrgb32, alpha: f32) -> Srgba32 {
        Srgba32 {
            r: nonlinearize32(c.r, GAMMA_32),
            g: nonlinearize32(c.g, GAMMA_32),
            b: nonlinearize32(c.b, GAMMA_32),
            a: alpha,
        }
    }
    /// Direct conversion to [`LinearSrgb32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_linear_srgb32(&self) -> LinearSrgb32 {
        LinearSrgb32 {
            r: linearize32(self.r, GAMMA_32),
            g: linearize32(self.g, GAMMA_32),
            b: linearize32(self.b, GAMMA_32),
        }
    }

    // LinearSrgba32

    /// Direct conversion from [`LinearSrgba32`].
    #[inline]
    pub fn from_linear_srgba32(c: LinearSrgba32) -> Srgba32 {
        Srgba32 {
            r: nonlinearize32(c.r, GAMMA_32),
            g: nonlinearize32(c.g, GAMMA_32),
            b: nonlinearize32(c.b, GAMMA_32),
            a: c.a,
        }
    }
    /// Direct conversion to [`LinearSrgb32`].
    #[inline]
    pub fn to_linear_srgba32(&self) -> LinearSrgba32 {
        LinearSrgba32 {
            r: linearize32(self.r, GAMMA_32),
            g: linearize32(self.g, GAMMA_32),
            b: linearize32(self.b, GAMMA_32),
            a: self.a,
        }
    }
}

/// # Indirect conversions
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl Srgba32 {
    // Oklab32

    /// Indirect conversion from [`Oklab32`], through [`LinearSrgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_oklab32(c: Oklab32, alpha: f32) -> Srgba32 {
        c.to_linear_srgb32().to_srgba32(alpha)
    }

    /// Indirect conversion to [`Oklab32`], through [`LinearSrgb32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_oklab32(&self) -> Oklab32 {
        self.to_linear_srgb32().to_oklab32()
    }

    // Oklch32

    /// Indirect conversion from [`Oklch32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_oklch32(c: Oklch32, alpha: f32) -> Srgba32 {
        c.to_oklab32().to_linear_srgb32().to_srgba32(alpha)
    }

    /// Indirect conversion to [`Oklch32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_oklch32(&self) -> Oklch32 {
        self.to_oklab32().to_oklch32()
    }
}

/* conversions: LinearSrgb32 */

/// # Direct conversions
impl LinearSrgb32 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub const fn from_array(c: [f32; 3]) -> LinearSrgb32 {
        LinearSrgb32 {
            r: c[0],
            g: c[1],
            b: c[2],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub const fn to_array(c: LinearSrgb32) -> [f32; 3] {
        [c.r, c.g, c.b]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub const fn from_tuple(c: (f32, f32, f32)) -> LinearSrgb32 {
        LinearSrgb32 {
            r: c.0,
            g: c.1,
            b: c.2,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub const fn to_tuple(c: LinearSrgb32) -> (f32, f32, f32) {
        (c.r, c.g, c.b)
    }
}

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl LinearSrgb32 {
    // Srgb32

    /// Direct conversion from [`Srgb32`].
    #[inline]
    pub fn from_srgb32(c: Srgb32) -> LinearSrgb32 {
        LinearSrgb32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
        }
    }

    /// Direct conversion to [`Srgb32`].
    #[inline]
    pub fn to_srgb32(&self) -> Srgb32 {
        Srgb32 {
            r: nonlinearize32(self.r, GAMMA_32),
            g: nonlinearize32(self.g, GAMMA_32),
            b: nonlinearize32(self.b, GAMMA_32),
        }
    }

    // Srgba32

    /// Direct conversion from [`Srgba32`].
    ///
    /// Loses the alpha channel
    #[inline]
    pub fn from_srgba32(c: Srgba32) -> LinearSrgb32 {
        LinearSrgb32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
        }
    }

    /// Direct conversion to [`Srgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba32(&self, alpha: f32) -> Srgba32 {
        Srgba32 {
            r: nonlinearize32(self.r, GAMMA_32),
            g: nonlinearize32(self.g, GAMMA_32),
            b: nonlinearize32(self.b, GAMMA_32),
            a: alpha,
        }
    }

    // LinearSrgba32

    /// Direct conversion from [`LinearSrgba32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn from_linear_srgba32(c: LinearSrgba32) -> LinearSrgb32 {
        LinearSrgb32 {
            r: c.r,
            g: c.g,
            b: c.b,
        }
    }

    /// Direct conversion to [`LinearSrgba32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_linear_srgba32(&self, alpha: f32) -> LinearSrgba32 {
        LinearSrgba32 {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    // Oklab32

    /// Direct conversion from [`Oklab32`].
    #[inline]
    pub fn from_oklab32(c: Oklab32) -> LinearSrgb32 {
        c.to_linear_srgb32()
    }

    /// Direct conversion to [`Oklab32`].
    #[inline]
    pub fn to_oklab32(&self) -> Oklab32 {
        Oklab32::from_linear_srgb32(*self)
    }
}

/// # Indirect conversions
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl LinearSrgb32 {
    // Srgb8

    /// Indirect conversion from [`Srgb8`].
    #[inline]
    pub fn from_srgb8(c: Srgb8) -> LinearSrgb32 {
        let c = c.to_srgb32();
        LinearSrgb32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
        }
    }

    /// Indirect conversion to [`Srgb32`].
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        self.to_srgb32().to_srgb8()
    }

    // Srgba8

    /// Indirect conversion from [`Srgba8`].
    ///
    /// Loses the alpha channel
    #[inline]
    pub fn from_srgba8(c: Srgba8) -> LinearSrgb32 {
        let c = c.to_srgba32();
        LinearSrgb32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
        }
    }

    /// Indirect conversion to [`Srgba8`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn to_srgba8(&self, alpha: u8) -> Srgba8 {
        self.to_srgb32().to_srgba8(alpha)
    }

    // Oklhc32

    /// Direct conversion from [`Oklch32`].
    #[inline]
    pub fn from_oklch32(c: Oklch32) -> LinearSrgb32 {
        c.to_oklab32().to_linear_srgb32()
    }

    /// Direct conversion to [`Oklab32`].
    #[inline]
    pub fn to_oklch32(&self) -> Oklch32 {
        Oklab32::from_linear_srgb32(*self).to_oklch32()
    }
}

/* conversions: LinearSrgba32 */

/// # Direct conversions
impl LinearSrgba32 {
    // [] ()

    /// Direct conversion from an array.
    #[inline]
    pub fn from_array(c: [f32; 4]) -> LinearSrgba32 {
        LinearSrgba32 {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
    /// Direct conversion to an array.
    #[inline]
    pub fn to_array(c: LinearSrgba32) -> [f32; 4] {
        [c.r, c.g, c.b, c.a]
    }

    /// Direct conversion from a tuple.
    #[inline]
    pub fn from_tuple(c: (f32, f32, f32, f32)) -> LinearSrgba32 {
        LinearSrgba32 {
            r: c.0,
            g: c.1,
            b: c.2,
            a: c.3,
        }
    }
    /// Direct conversion to a tuple.
    #[inline]
    pub fn to_tuple(c: LinearSrgba32) -> (f32, f32, f32, f32) {
        (c.r, c.g, c.b, c.a)
    }
}

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl LinearSrgba32 {
    // Srgb32

    /// Direct conversion from [`Srgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_srgb32(c: Srgb32, alpha: f32) -> LinearSrgba32 {
        LinearSrgba32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
            a: alpha,
        }
    }

    /// Direct conversion to [`Srgb32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_srgb32(&self) -> Srgb32 {
        Srgb32 {
            r: nonlinearize32(self.r, GAMMA_32),
            g: nonlinearize32(self.g, GAMMA_32),
            b: nonlinearize32(self.b, GAMMA_32),
        }
    }

    // Srgba32

    /// Direct conversion from [`Srgba32`].
    #[inline]
    pub fn from_srgba32(c: Srgba32) -> LinearSrgba32 {
        LinearSrgba32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
            a: c.a,
        }
    }

    /// Direct conversion to [`Srgba32`].
    #[inline]
    pub fn to_srgba32(&self) -> Srgba32 {
        Srgba32 {
            r: nonlinearize32(self.r, GAMMA_32),
            g: nonlinearize32(self.g, GAMMA_32),
            b: nonlinearize32(self.b, GAMMA_32),
            a: self.a,
        }
    }

    // LinearSrgb32

    /// Direct conversion from [`LinearSrgb32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_linear_srgb32(c: LinearSrgb32, alpha: f32) -> LinearSrgba32 {
        LinearSrgba32 {
            r: c.r,
            g: c.g,
            b: c.b,
            a: alpha,
        }
    }

    /// Direct conversion to [`LinearSrgb32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_linear_srgb32(&self) -> LinearSrgb32 {
        LinearSrgb32 {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }

    // Oklab32

    /// Direct conversion from [`Oklab32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_oklab32(c: Oklab32, alpha: f32) -> LinearSrgba32 {
        c.to_linear_srgba32(alpha)
    }

    /// Direct conversion to [`Oklab32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_oklab32(&self) -> Oklab32 {
        Oklab32::from_linear_srgba32(*self)
    }
}

/// # Indirect conversions
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
impl LinearSrgba32 {
    // Srgb8

    /// Indirect conversion from [`Srgb8`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_srgb8(c: Srgb8, alpha: f32) -> LinearSrgba32 {
        let c = c.to_srgb32();
        LinearSrgba32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
            a: alpha,
        }
    }

    /// Indirect conversion to [`Srgb32`].
    ///
    /// Loses the alpha channel
    #[inline]
    pub fn to_srgb8(&self) -> Srgb8 {
        Srgb32 {
            r: nonlinearize32(self.r, GAMMA_32),
            g: nonlinearize32(self.g, GAMMA_32),
            b: nonlinearize32(self.b, GAMMA_32),
        }
        .to_srgb8()
    }

    // Srgba8

    /// Indirect conversion from [`Srgba8`].
    #[inline]
    pub fn from_srgba8(c: Srgba8) -> LinearSrgba32 {
        let c = c.to_srgba32();
        LinearSrgba32 {
            r: linearize32(c.r, GAMMA_32),
            g: linearize32(c.g, GAMMA_32),
            b: linearize32(c.b, GAMMA_32),
            a: c.a,
        }
    }

    /// Indirect conversion to [`Srgba8`].
    #[inline]
    pub fn to_srgba8(&self) -> Srgba8 {
        Srgba32 {
            r: nonlinearize32(self.r, GAMMA_32),
            g: nonlinearize32(self.g, GAMMA_32),
            b: nonlinearize32(self.b, GAMMA_32),
            a: self.a,
        }
        .to_srgba8()
    }

    // Oklhc32

    /// Direct conversion from [`Oklch32`].
    ///
    /// Adds the `alpha` channel.
    #[inline]
    pub fn from_oklch32(c: Oklch32, alpha: f32) -> LinearSrgba32 {
        c.to_oklab32().to_linear_srgba32(alpha)
    }

    /// Direct conversion to [`Oklab32`].
    ///
    /// Loses the alpha channel.
    #[inline]
    pub fn to_oklch32(&self) -> Oklch32 {
        Oklab32::from_linear_srgba32(*self).to_oklch32()
    }
}

// From/Into impls
// -----------------------------------------------------------------------------

#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
mod impl_from {
    use super::*;

    /* From Srgb8 */

    impl From<Srgb8> for Srgba8 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Srgb8) -> Srgba8 {
            c.to_srgba8(u8::MAX)
        }
    }
    impl From<Srgb8> for Srgb32 {
        #[inline]
        fn from(c: Srgb8) -> Srgb32 {
            c.to_srgb32()
        }
    }
    impl From<Srgb8> for Srgba32 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Srgb8) -> Srgba32 {
            c.to_srgba32(1.)
        }
    }
    impl From<Srgb8> for Oklab32 {
        #[inline]
        fn from(c: Srgb8) -> Oklab32 {
            c.to_oklab32()
        }
    }
    impl From<Srgb8> for Oklch32 {
        #[inline]
        fn from(c: Srgb8) -> Oklch32 {
            c.to_oklab32().to_oklch32()
        }
    }
    // … LinearSrgb32, LinearSrgba32

    /* From Srgba8 */

    impl From<Srgba8> for Srgb8 {
        #[inline]
        fn from(c: Srgba8) -> Srgb8 {
            c.to_srgb8()
        }
    }
    impl From<Srgba8> for Srgb32 {
        #[inline]
        fn from(c: Srgba8) -> Srgb32 {
            c.to_srgb32()
        }
    }
    impl From<Srgba8> for Srgba32 {
        #[inline]
        fn from(c: Srgba8) -> Srgba32 {
            c.to_srgba32()
        }
    }
    impl From<Srgba8> for Oklab32 {
        #[inline]
        fn from(c: Srgba8) -> Oklab32 {
            c.to_oklab32()
        }
    }
    impl From<Srgba8> for Oklch32 {
        #[inline]
        fn from(c: Srgba8) -> Oklch32 {
            c.to_oklab32().to_oklch32()
        }
    }
    // … LinearSrgb32, LinearSrgba32

    /* From Srgb32 */

    impl From<Srgb32> for Srgb8 {
        #[inline]
        fn from(c: Srgb32) -> Srgb8 {
            c.to_srgb8()
        }
    }
    impl From<Srgb32> for Srgba8 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Srgb32) -> Srgba8 {
            c.to_srgba8(u8::MAX)
        }
    }
    impl From<Srgb32> for Srgba32 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Srgb32) -> Srgba32 {
            c.to_srgba32(1.)
        }
    }
    impl From<Srgb32> for LinearSrgb32 {
        #[inline]
        fn from(c: Srgb32) -> LinearSrgb32 {
            c.to_linear_srgb32()
        }
    }
    impl From<Srgb32> for LinearSrgba32 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: Srgb32) -> LinearSrgba32 {
            c.to_linear_srgba32(1.)
        }
    }
    impl From<Srgb32> for Oklab32 {
        #[inline]
        fn from(c: Srgb32) -> Oklab32 {
            c.to_oklab32()
        }
    }
    impl From<Srgb32> for Oklch32 {
        #[inline]
        fn from(c: Srgb32) -> Oklch32 {
            c.to_oklab32().to_oklch32()
        }
    }

    /* From LinearSrgb32 */

    impl From<LinearSrgb32> for Srgb8 {
        #[inline]
        fn from(c: LinearSrgb32) -> Srgb8 {
            c.to_srgb32().to_srgb8()
        }
    }
    impl From<LinearSrgb32> for Srgba8 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: LinearSrgb32) -> Srgba8 {
            c.to_srgb32().to_srgba8(u8::MAX)
        }
    }
    impl From<LinearSrgb32> for Srgb32 {
        #[inline]
        fn from(c: LinearSrgb32) -> Srgb32 {
            c.to_srgb32()
        }
    }
    impl From<LinearSrgb32> for Srgba32 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: LinearSrgb32) -> Srgba32 {
            c.to_srgba32(1.)
        }
    }
    impl From<LinearSrgb32> for LinearSrgba32 {
        /// Automatically adds alpha at max opacity.
        #[inline]
        fn from(c: LinearSrgb32) -> LinearSrgba32 {
            c.to_linear_srgba32(1.)
        }
    }
    impl From<LinearSrgb32> for Oklab32 {
        #[inline]
        fn from(c: LinearSrgb32) -> Oklab32 {
            c.to_oklab32()
        }
    }
    impl From<LinearSrgb32> for Oklch32 {
        #[inline]
        fn from(c: LinearSrgb32) -> Oklch32 {
            c.to_oklab32().to_oklch32()
        }
    }

    /* From LinearSrgba32 */

    impl From<LinearSrgba32> for Srgb8 {
        #[inline]
        fn from(c: LinearSrgba32) -> Srgb8 {
            c.to_srgb32().to_srgb8()
        }
    }
    impl From<LinearSrgba32> for Srgba8 {
        #[inline]
        fn from(c: LinearSrgba32) -> Srgba8 {
            c.to_srgba32().to_srgba8()
        }
    }
    impl From<LinearSrgba32> for Srgb32 {
        #[inline]
        fn from(c: LinearSrgba32) -> Srgb32 {
            c.to_srgb32()
        }
    }
    impl From<LinearSrgba32> for Srgba32 {
        #[inline]
        fn from(c: LinearSrgba32) -> Srgba32 {
            c.to_srgba32()
        }
    }
    impl From<LinearSrgba32> for LinearSrgb32 {
        #[inline]
        fn from(c: LinearSrgba32) -> LinearSrgb32 {
            c.to_linear_srgb32()
        }
    }
    impl From<LinearSrgba32> for Oklab32 {
        #[inline]
        fn from(c: LinearSrgba32) -> Oklab32 {
            c.to_oklab32()
        }
    }
    impl From<LinearSrgba32> for Oklch32 {
        #[inline]
        fn from(c: LinearSrgba32) -> Oklch32 {
            c.to_oklab32().to_oklch32()
        }
    }
}

/* utils */

/// Applies the `gamma` to an `f32` channel.
#[inline]
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
pub fn linearize32(nonlinear: f32, gamma: f32) -> f32 {
    if nonlinear >= 0.04045 {
        #[cfg(feature = "std")]
        return ((nonlinear + 0.055) / (1. + 0.055)).powf(gamma);
        #[cfg(not(feature = "std"))]
        return libm::powf((nonlinear + 0.055) / (1. + 0.055), gamma);
    } else {
        nonlinear / 12.92
    }
}

/// Removes the `gamma` from an `f32` channel.
#[inline]
#[cfg(any(feature = "std", feature = "no-std"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", feature = "no-std")))
)]
pub fn nonlinearize32(linear: f32, gamma: f32) -> f32 {
    if linear >= 0.0031308 {
        #[cfg(feature = "std")]
        return (1.055) * linear.powf(1.0 / gamma) - 0.055;
        #[cfg(not(feature = "std"))]
        return (1.055) * libm::powf(linear, 1.0 / gamma) - 0.055;
    } else {
        12.92 * linear
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
