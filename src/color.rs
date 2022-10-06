// acolor::color
//
//!
//

use super::{LinearSrgb32, LinearSrgba32, Oklab32, Oklch32, Srgb32, Srgb8, Srgba32, Srgba8};
use iunorm::Unorm8;

/// Common color trait for all supported color formats.
pub trait Color {
    /// The type of the inner color components.
    type Inner;

    /// Returns the red luminosity.
    fn color_red(&self) -> Self::Inner;
    /// Returns the green luminosity.
    fn color_green(&self) -> Self::Inner;
    /// Returns the blue luminosity.
    fn color_blue(&self) -> Self::Inner;
    /// Returns the alpha luminosity.
    fn color_alpha(&self) -> Self::Inner;

    /// Returns the overall luminosity.
    ///
    /// If the color is not in [`Oklab32`] or [`Oklch32`] format,
    /// it will be converted to `Oklab32` for the operation.
    fn color_luminosity(&self) -> Self::Inner;

    /// Returns the hue.
    ///
    /// If the color is not in [`Oklch32`] format
    /// it will be converted to it for the operation.
    fn color_hue(&self) -> Self::Inner;

    /* conversions */

    /// Returns the 3 components, without alpha.
    fn color_to_array3(&self) -> [Self::Inner; 3];
    /// Returns the 4 components, with alpha.
    ///
    /// If the specific color type has no alpha the maximum value is returned.
    fn color_to_array4(&self) -> [Self::Inner; 4];


    /// Conversion to `Srgb8`.
    fn color_to_srgb8(&self) -> Srgb8;
    /// Conversion to `Srgba8`.
    fn color_to_srgba8(&self) -> Srgba8;
    /// Conversion to `Srgb32`.
    fn color_to_srgb32(&self) -> Srgb32;
    /// Conversion to `Srgba32`.
    fn color_to_srgba32(&self) -> Srgba32;
    /// Conversion to `LinearSrgb32`.
    fn color_to_linear_srgb32(&self) -> LinearSrgb32;
    /// Conversion to `LinearSrgba32`.
    fn color_to_linear_srgba32(&self) -> LinearSrgba32;
    /// Conversion to `Oklab32`.
    fn color_to_oklab32(&self) -> Oklab32;
    /// Conversion to `Oklch32`.
    fn color_to_oklch32(&self) -> Oklch32;
}

#[rustfmt::skip]
impl Color for Srgb8 {
    type Inner = u8;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.r, self.g, self.b] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.r, self.g, self.b, u8::MAX] }

    /// Returns the gamma corrected red luminosity.
    fn color_red(&self) -> Self::Inner { self.r }
    /// Returns the gamma corrected green luminosity.
    fn color_green(&self) -> Self::Inner { self.g }
    /// Returns the gamma corrected blue luminosity.
    fn color_blue(&self) -> Self::Inner { self.b }
    /// Returns the maximum opacity alpha.
    fn color_alpha(&self) -> Self::Inner { u8::MAX }
    fn color_luminosity(&self) -> Self::Inner { Unorm8::from_f32(self.to_oklab32().l).0 }
    fn color_hue(&self) -> Self::Inner { Unorm8::from_f32(self.to_oklch32().h).0 }

    /// no-op.
    fn color_to_srgb8(&self) -> Srgb8 { *self }
    fn color_to_srgba8(&self) -> Srgba8 { self.to_srgba8(u8::MAX) }
    fn color_to_srgb32(&self) -> Srgb32 { self.to_srgb32() }
    fn color_to_srgba32(&self) -> Srgba32 { self.to_srgba32(1.) }
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { self.to_linear_srgb32() }
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { self.to_linear_srgba32(1.) }
    fn color_to_oklab32(&self) -> Oklab32 { self.to_oklab32() }
    fn color_to_oklch32(&self) -> Oklch32 { self.to_oklch32() }
}
#[rustfmt::skip]
impl Color for Srgba8 {
    type Inner = u8;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.r, self.g, self.b] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.r, self.g, self.b, self.a] }

    /// Returns the gamma corrected red luminosity.
    fn color_red(&self) -> Self::Inner { self.r }
    /// Returns the gamma corrected green luminosity.
    fn color_green(&self) -> Self::Inner { self.g }
    /// Returns the gamma corrected blue luminosity.
    fn color_blue(&self) -> Self::Inner { self.b }
    /// Returns the linear alpha.
    fn color_alpha(&self) -> Self::Inner { self.a }
    fn color_luminosity(&self) -> Self::Inner { Unorm8::from_f32(self.to_oklab32().l).0 }
    fn color_hue(&self) -> Self::Inner { Unorm8::from_f32(self.to_oklch32().h).0 }

    fn color_to_srgb8(&self) -> Srgb8 { self.to_srgb8() }
    /// no-op.
    fn color_to_srgba8(&self) -> Srgba8 { *self }
    fn color_to_srgb32(&self) -> Srgb32 { self.to_srgb32() }
    fn color_to_srgba32(&self) -> Srgba32 { self.to_srgba32() }
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { self.to_linear_srgb32() }
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { self.to_linear_srgba32() }
    fn color_to_oklab32(&self) -> Oklab32 { self.to_oklab32() }
    fn color_to_oklch32(&self) -> Oklch32 { self.to_oklch32() }
}
#[rustfmt::skip]
impl Color for Srgb32 {
    type Inner = f32;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.r, self.g, self.b] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.r, self.g, self.b, 1.] }

    /// Returns the gamma corrected red luminosity.
    fn color_red(&self) -> Self::Inner { self.r }
    /// Returns the gamma corrected green luminosity.
    fn color_green(&self) -> Self::Inner { self.g }
    /// Returns the gamma corrected blue luminosity.
    fn color_blue(&self) -> Self::Inner { self.b }
    /// Returns the maximum opacity alpha.
    fn color_alpha(&self) -> Self::Inner { 1. }
    fn color_luminosity(&self) -> Self::Inner { self.to_oklab32().l }
    fn color_hue(&self) -> Self::Inner { self.to_oklch32().h }

    fn color_to_srgb8(&self) -> Srgb8 { self.to_srgb8() }
    fn color_to_srgba8(&self) -> Srgba8 { self.to_srgba8(u8::MAX) }
    /// no-op.
    fn color_to_srgb32(&self) -> Srgb32 { *self }
    fn color_to_srgba32(&self) -> Srgba32 { self.to_srgba32(1.) }
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { self.to_linear_srgb32() }
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { self.to_linear_srgba32(1.) }
    fn color_to_oklab32(&self) -> Oklab32 { self.to_oklab32() }
    fn color_to_oklch32(&self) -> Oklch32 { self.to_oklch32() }
}
#[rustfmt::skip]
impl Color for Srgba32 {
    type Inner = f32;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.r, self.g, self.b] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.r, self.g, self.b, self.a] }

    /// Returns the gamma corrected red luminosity.
    fn color_red(&self) -> Self::Inner { self.r }
    /// Returns the gamma corrected green luminosity.
    fn color_green(&self) -> Self::Inner { self.g }
    /// Returns the gamma corrected blue luminosity.
    fn color_blue(&self) -> Self::Inner { self.b }
    /// Returns the linear alpha.
    fn color_alpha(&self) -> Self::Inner { self.a }
    fn color_luminosity(&self) -> Self::Inner { self.to_oklab32().l }
    fn color_hue(&self) -> Self::Inner { self.to_oklch32().h }

    fn color_to_srgb8(&self) -> Srgb8 { self.to_srgb8() }
    fn color_to_srgba8(&self) -> Srgba8 { self.to_srgba8() }
    fn color_to_srgb32(&self) -> Srgb32 { self.to_srgb32() }
    /// no-op.
    fn color_to_srgba32(&self) -> Srgba32 { *self }
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { self.to_linear_srgb32() }
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { self.to_linear_srgba32() }
    fn color_to_oklab32(&self) -> Oklab32 { self.to_oklab32() }
    fn color_to_oklch32(&self) -> Oklch32 { self.to_oklch32() }
}
#[rustfmt::skip]
impl Color for LinearSrgb32 {
    type Inner = f32;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.r, self.g, self.b] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.r, self.g, self.b, 1.] }

    /// Returns the linear red luminosity.
    fn color_red(&self) -> Self::Inner { self.r }
    /// Returns the linear green luminosity.
    fn color_green(&self) -> Self::Inner { self.g }
    /// Returns the linear blue luminosity.
    fn color_blue(&self) -> Self::Inner { self.b }
    /// Returns the maximum opacity alpha.
    fn color_alpha(&self) -> Self::Inner { 1. }
    fn color_luminosity(&self) -> Self::Inner { self.to_oklab32().l }
    fn color_hue(&self) -> Self::Inner { self.to_oklch32().h }

    fn color_to_srgb8(&self) -> Srgb8 { self.to_srgb8() }
    fn color_to_srgba8(&self) -> Srgba8 { self.to_srgba8(u8::MAX) }
    fn color_to_srgb32(&self) -> Srgb32 { self.to_srgb32() }
    fn color_to_srgba32(&self) -> Srgba32 { self.to_srgba32(1.) }
    /// no-op.
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { *self }
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { self.to_linear_srgba32(1.) }
    fn color_to_oklab32(&self) -> Oklab32 { self.to_oklab32() }
    fn color_to_oklch32(&self) -> Oklch32 { self.to_oklch32() }
}
#[rustfmt::skip]
impl Color for LinearSrgba32 {
    type Inner = f32;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.r, self.g, self.b] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.r, self.g, self.b, self.a] }

    /// Returns the linear red luminosity.
    fn color_red(&self) -> Self::Inner { self.r }
    /// Returns the linear green luminosity.
    fn color_green(&self) -> Self::Inner { self.g }
    /// Returns the linear blue luminosity.
    fn color_blue(&self) -> Self::Inner { self.b }
    /// Returns the linear alpha.
    fn color_alpha(&self) -> Self::Inner { self.a }
    fn color_luminosity(&self) -> Self::Inner { self.to_oklab32().l }
    fn color_hue(&self) -> Self::Inner { self.to_oklch32().h }

    fn color_to_srgb8(&self) -> Srgb8 { self.to_srgb8() }
    fn color_to_srgba8(&self) -> Srgba8 { self.to_srgba8() }
    fn color_to_srgb32(&self) -> Srgb32 { self.to_srgb32() }
    fn color_to_srgba32(&self) -> Srgba32 { self.to_srgba32() }
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { self.to_linear_srgb32() }
    /// no-op.
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { *self }
    fn color_to_oklab32(&self) -> Oklab32 { self.to_oklab32() }
    fn color_to_oklch32(&self) -> Oklch32 { self.to_oklch32() }
}
#[rustfmt::skip]
impl Color for Oklab32 {
    type Inner = f32;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.l, self.a, self.b] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.l, self.a, self.b, 1.] }

    /// Returns the linear red luminosity, after converting to [`LinearSrgb32`].
    fn color_red(&self) -> Self::Inner { self.color_to_linear_srgb32().r }
    /// Returns the linear green luminosity, after converting to [`LinearSrgb32`].
    fn color_green(&self) -> Self::Inner { self.color_to_linear_srgb32().g }
    /// Returns the linear blue luminosity, after converting to [`LinearSrgb32`].
    fn color_blue(&self) -> Self::Inner { self.color_to_linear_srgb32().b }
    /// Returns the maximum opacity alpha.
    fn color_alpha(&self) -> Self::Inner { 1. }
    fn color_luminosity(&self) -> Self::Inner { self.l }
    fn color_hue(&self) -> Self::Inner { self.to_oklch32().h }

    fn color_to_srgb8(&self) -> Srgb8 { self.to_srgb8() }
    fn color_to_srgba8(&self) -> Srgba8 { self.to_srgba8(u8::MAX) }
    fn color_to_srgb32(&self) -> Srgb32 { self.to_srgb32() }
    fn color_to_srgba32(&self) -> Srgba32 { self.to_srgba32(1.) }
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { self.to_linear_srgb32() }
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { self.to_linear_srgba32(1.) }
    /// no-op.
    fn color_to_oklab32(&self) -> Oklab32 { *self }
    fn color_to_oklch32(&self) -> Oklch32 { self.to_oklch32() }
}
#[rustfmt::skip]
impl Color for Oklch32 {
    type Inner = f32;
    fn color_to_array3(&self) -> [Self::Inner; 3] { [self.l, self.c, self.h] }
    fn color_to_array4(&self) -> [Self::Inner; 4] { [self.l, self.c, self.h, 1.] }

    /// Returns the red luminosity, after converting to [`LinearSrgb32`].
    fn color_red(&self) -> Self::Inner { self.color_to_linear_srgb32().r }
    /// Returns the green luminosity, after converting to [`LinearSrgb32`].
    fn color_green(&self) -> Self::Inner { self.color_to_linear_srgb32().g }
    /// Returns the blue luminosity, after converting to [`LinearSrgb32`].
    fn color_blue(&self) -> Self::Inner { self.color_to_linear_srgb32().b }
    /// Returns the maximum opacity alpha.
    fn color_alpha(&self) -> Self::Inner { 1. }
    fn color_luminosity(&self) -> Self::Inner { self.l }
    fn color_hue(&self) -> Self::Inner { self.h }

    fn color_to_srgb8(&self) -> Srgb8 { self.to_srgb8() }
    fn color_to_srgba8(&self) -> Srgba8 { self.to_srgba8(u8::MAX) }
    fn color_to_srgb32(&self) -> Srgb32 { self.to_srgb32() }
    fn color_to_srgba32(&self) -> Srgba32 { self.to_srgba32(1.) }
    fn color_to_linear_srgb32(&self) -> LinearSrgb32 { self.to_linear_srgb32() }
    fn color_to_linear_srgba32(&self) -> LinearSrgba32 { self.to_linear_srgba32(1.) }
    fn color_to_oklab32(&self) -> Oklab32 { self.to_oklab32() }
    /// no-op.
    fn color_to_oklch32(&self) -> Oklch32 { *self }
}
