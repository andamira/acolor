// acolor::color
//
//!
//

/// The default gamma value as an [`f32`].
pub const GAMMA_32: f32 = 2.4;

/* color types */

pub mod srgb;
pub use srgb::{linear_srgb_to_srgb_32, srgb_to_linear_srgb_32, LinearSrgb32, Srgb32, Srgb8};

pub mod oklab;
pub use oklab::{linear_srgb_to_oklab_32, oklab_to_linear_srgb_32, Oklab32};

pub mod oklch;
pub use oklch::{oklab_to_oklch_32, oklch_to_oklab_32, Oklch32};

// mod aces;
// pub use aces::*;
