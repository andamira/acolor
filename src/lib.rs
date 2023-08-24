// acolor
//
//! A handy selection of color representations, conversions and operations.
//
// # Color components
//
// - *Chroma* is the perceived strength of a color, in relation to white.
//
// # Color primaries
//
// - red, green, blue
// - cyan, magenta, yellow
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
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't have both the `std` and `no-std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");


/* color trait */

mod color;
pub use color::Color;

/* color types */

pub mod srgb;
#[doc(inline)]
#[cfg(any(feature = "std", feature = "no-std"))]
pub use srgb::{linearize32, nonlinearize32};
#[doc(inline)]
pub use srgb::{LinearSrgb32, LinearSrgba32, Srgb32, Srgb8, Srgba32, Srgba8};

pub mod oklab;
#[doc(inline)]
pub use oklab::{Oklab32, Oklch32};

mod external;

#[cfg(test)]
mod tests;

/* constants */

/// The default gamma value as an [`f32`].
pub const GAMMA_32: f32 = 2.4;
