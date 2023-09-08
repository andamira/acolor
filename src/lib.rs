// acolor
//
//! Color representations, operations and conversions.
//
// # Color components
//
// - *Chroma* is the perceived strength of a color, in relation to white.
//
// # Color primaries
//
// - red, green, blue
// - cyan, magenta, yellow

// warnings
#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::excessive_precision,
    clippy::pattern_type_mismatch
)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't have both the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");
// deprecated
devela::deprecate_feature![old: "no-std", new: "no_std", since: "0.0.10"];
devela::deprecate_feature![old: "complete", new: "full", since: "0.0.11"];

mod external; // trait impls on external types
#[cfg(test)]
mod tests;

mod color;
mod gamma;
pub mod oklab;
pub mod srgb;

pub use {color::*, gamma::*};

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{color::Color, gamma::*, oklab::*, srgb::*};
}
