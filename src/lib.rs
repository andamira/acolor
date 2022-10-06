// ::
//! A handy selection of color representations, conversions and operations.
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::excessive_precision,
    clippy::pattern_type_mismatch
)]
#![forbid(unsafe_code)]
//
#![cfg_attr(not(feature = "std"), no_std)]

/* constants */

/// The default gamma value as an [`f32`].
pub const GAMMA_32: f32 = 2.4;

/* color trait */

mod color;
pub use color::Color;

/* color types */

pub mod srgb;
#[doc(inline)]
pub use srgb::{
    linearize32, nonlinearize32, LinearSrgb32, LinearSrgba32, Srgb32, Srgb8, Srgba32, Srgba8,
};

pub mod oklab;
#[doc(inline)]
pub use oklab::{Oklab32, Oklch32};

mod external;

#[cfg(test)]
mod tests;

/* utils */

/// Min function using `PartialOrd`.
#[inline(always)]
#[rustfmt::skip]
pub(crate) fn min<T: PartialOrd>(a: T, b: T) -> T { if a < b { a } else { b } }

/// Max function using `PartialOrd`.
#[inline(always)]
#[rustfmt::skip]
pub(crate) fn max<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

/// Minmax function using `PartialOrd`.
#[inline(always)]
#[rustfmt::skip]
pub(crate) fn clamp<T: PartialOrd>(value: T, vmin: T, vmax: T) -> T {
    min(max(value, vmin), vmax)
}
