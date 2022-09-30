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

/* color types */

pub mod srgb;
#[doc(inline)]
pub use srgb::{
    linear_srgb_to_srgb_32, linearize_32, nonlinearize_32, srgb_to_linear_srgb_32, LinearSrgb32,
    LinearSrgba32, Srgb32, Srgb8, Srgba32, Srgba8,
};

pub mod oklab;
#[doc(inline)]
pub use oklab::{
    linear_srgb_to_oklab_32, oklab_to_linear_srgb_32, oklab_to_oklch_32, oklch_to_oklab_32,
    Oklab32, Oklch32,
};

mod external;

// mod aces;
// pub use aces::*;

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

#[cfg(test)]
mod tests {
    use super::{clamp, max, min};

    #[test]
    fn test_clamp() {
        assert_eq![2, min(2, 5)];
        assert_eq![2, min(5, 2)];
        assert_eq![2., min(2., 5.)];

        assert_eq![5, max(2, 5)];
        assert_eq![5, max(5, 2)];
        assert_eq![5., max(2., 5.)];

        assert_eq![3, clamp(3, 2, 5)];
        assert_eq![3., clamp(3., 2., 5.)];
        assert_eq![2, clamp(1, 2, 5)];
        assert_eq![5, clamp(7, 2, 5)];
    }
}
