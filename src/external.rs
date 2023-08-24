// acolor::external
//
//! Optional external traits implementations and type conversions.
//
// TOC
// - macroquad
// - sdl2
// - tiny-skia
// - notcurses
// - approx
//

#[cfg(feature = "rgb")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "rgb")))]
mod rgb {
    use crate::{Srgb32, Srgb8, Srgba32, Srgba8};
    use rgb::{RGB, RGBA};

    /// Convert rust-rgb's `RGB<u8>` type into `Srgb8`.
    impl From<RGB<u8>> for Srgb8 {
        fn from(item: RGB<u8>) -> Self {
            Self::new(item.r, item.g, item.b)
        }
    }
    /// Convert `Srgb8` into rust-rgb's `RGB<u8>` type.
    impl From<Srgb8> for RGB<u8> {
        fn from(item: Srgb8) -> Self {
            Self::new(item.r, item.g, item.b)
        }
    }

    /// Convert rust-rgb's `RGBA<u8>` type into `Srgba8`.
    impl From<RGBA<u8>> for Srgba8 {
        fn from(item: RGBA<u8>) -> Self {
            Self::new(item.r, item.g, item.b, item.a)
        }
    }
    /// Convert `Srgba8` into rust-rgb's `RGBA<u8>` type.
    impl From<Srgba8> for RGBA<u8> {
        fn from(item: Srgba8) -> Self {
            Self::new(item.r, item.g, item.b, item.a)
        }
    }

    //

    /// Convert rust-rgb's `RGB<f32>` type into `Srgb32`.
    impl From<RGB<f32>> for Srgb32 {
        fn from(item: RGB<f32>) -> Self {
            Self::new(item.r, item.g, item.b)
        }
    }
    /// Convert `Srgb32` into rust-rgb's `RGB<f32>` type.
    impl From<Srgb32> for RGB<f32> {
        fn from(item: Srgb32) -> Self {
            Self::new(item.r, item.g, item.b)
        }
    }

    /// Convert rust-rgb's `RGBA<f32>` type into `Srgba32`.
    impl From<RGBA<f32>> for Srgba32 {
        fn from(item: RGBA<f32>) -> Self {
            Self::new(item.r, item.g, item.b, item.a)
        }
    }
    /// Convert `Srgba32` into rust-rgb's `RGBA<f32>` type.
    impl From<Srgba32> for RGBA<f32> {
        fn from(item: Srgba32) -> Self {
            Self::new(item.r, item.g, item.b, item.a)
        }
    }
}

#[cfg(feature = "macroquad")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "macroquad")))]
mod macroquad {
    use crate::{Srgb32, Srgb8, Srgba32, Srgba8};
    pub use macroquad::color::Color;

    // u8

    impl From<Srgb8> for Color {
        /// Into [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Srgb8) -> Color {
            Color::from_rgba(c.r, c.g, c.b, 255)
        }
    }
    impl From<Color> for Srgb8 {
        /// From [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Color) -> Srgb8 {
            Srgb32::new(c.r, c.g, c.b).to_srgb8()
        }
    }

    impl From<Srgba8> for Color {
        /// Into [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Srgba8) -> Color {
            Color::from_rgba(c.r, c.g, c.b, c.a)
        }
    }
    impl From<Color> for Srgba8 {
        /// From [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Color) -> Srgba8 {
            Srgba32::new(c.r, c.g, c.b, c.a).to_srgba8()
        }
    }
    // f32

    impl From<Srgb32> for Color {
        /// Into [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Srgb32) -> Color {
            Color::new(c.r, c.g, c.b, 1.)
        }
    }
    impl From<Color> for Srgb32 {
        /// From [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Color) -> Srgb32 {
            Srgb32::new(c.r, c.g, c.b)
        }
    }

    impl From<Srgba32> for Color {
        /// Into [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Srgba32) -> Color {
            Color::new(c.r, c.g, c.b, c.a)
        }
    }
    impl From<Color> for Srgba32 {
        /// From [macroquad's `Color`][0].
        ///
        /// [0]: https://docs.rs/macroquad/latest/macroquad/color/struct.Color.html
        fn from(c: Color) -> Srgba32 {
            Srgba32::new(c.r, c.g, c.b, c.a)
        }
    }
}

#[cfg(feature = "sdl2")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sdl2")))]
mod sdl2 {
    use crate::{Srgb8, Srgba8};
    use sdl2::gfx::primitives::ToColor;
    use sdl2::pixels::Color;

    impl From<Srgb8> for Color {
        /// Into [sdl2's `Color`][0].
        ///
        /// [0]: https://docs.rs/sdl2/latest/sdl2/pixels/struct.Color.html
        fn from(c: Srgb8) -> Color {
            Color {
                r: c.r,
                g: c.g,
                b: c.b,
                a: 255,
            }
        }
    }
    impl From<Color> for Srgb8 {
        /// From [sdl2's `Color`][0].
        ///
        /// [0]: https://docs.rs/sdl2/latest/sdl2/pixels/struct.Color.html
        fn from(c: Color) -> Srgb8 {
            Srgb8::new(c.r, c.g, c.b)
        }
    }
    impl ToColor for Srgb8 {
        /// Automatically adds alpha at max opacity.
        fn as_rgba(&self) -> (u8, u8, u8, u8) {
            (self.r, self.g, self.b, u8::MAX)
        }
    }

    impl From<Srgba8> for Color {
        /// Into [sdl2's `Color`][0].
        ///
        /// [0]: https://docs.rs/sdl2/latest/sdl2/pixels/struct.Color.html
        fn from(c: Srgba8) -> Color {
            Color {
                r: c.r,
                g: c.g,
                b: c.b,
                a: c.a,
            }
        }
    }
    impl From<Color> for Srgba8 {
        /// From [sdl2's `Color`][0].
        ///
        /// [0]: https://docs.rs/sdl2/latest/sdl2/pixels/struct.Color.html
        fn from(c: Color) -> Srgba8 {
            Srgba8::new(c.r, c.g, c.b, c.a)
        }
    }
    impl ToColor for Srgba8 {
        fn as_rgba(&self) -> (u8, u8, u8, u8) {
            (self.r, self.g, self.b, self.a)
        }
    }
}

// NOTE: tiny-skia fails to compile before we reach this point.
#[cfg(all(feature = "tiny-skia", any(feature = "std", feature = "no-std")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "tiny-skia", any(feature = "std", feature = "no-std"))))
)]
mod tiny_skia {
    use crate::{Srgb32, Srgb8, Srgba32, Srgba8};
    use devela::cmp::pclamp;
    use tiny_skia::{Color, ColorU8};
    use tiny_skia::{PremultipliedColor as PmColor, PremultipliedColorU8 as PmColorU8};

    // u8

    impl From<Srgb8> for ColorU8 {
        fn from(c: Srgb8) -> ColorU8 {
            ColorU8::from_rgba(c.r, c.g, c.b, 255)
        }
    }
    impl From<ColorU8> for Srgb8 {
        fn from(c: ColorU8) -> Srgb8 {
            Srgb8::new(c.red(), c.green(), c.blue())
        }
    }

    impl From<Srgba8> for ColorU8 {
        fn from(c: Srgba8) -> ColorU8 {
            ColorU8::from_rgba(c.r, c.g, c.b, c.a)
        }
    }
    impl From<ColorU8> for Srgba8 {
        fn from(c: ColorU8) -> Srgba8 {
            Srgba8::new(c.red(), c.green(), c.blue(), c.alpha())
        }
    }

    // u8 (premultiplied)

    impl From<Srgb8> for PmColorU8 {
        fn from(c: Srgb8) -> PmColorU8 {
            ColorU8::from(c).premultiply()
        }
    }
    impl From<PmColorU8> for Srgb8 {
        fn from(c: PmColorU8) -> Srgb8 {
            c.demultiply().into()
        }
    }

    impl From<Srgba8> for PmColorU8 {
        fn from(c: Srgba8) -> PmColorU8 {
            ColorU8::from(c).premultiply()
        }
    }
    impl From<PmColorU8> for Srgba8 {
        fn from(c: PmColorU8) -> Srgba8 {
            c.demultiply().into()
        }
    }

    // f32

    impl From<Srgb32> for Color {
        fn from(c: Srgb32) -> Color {
            Color::from_rgba(c.r, c.g, c.b, 1.).unwrap_or_else(|| {
                Color::from_rgba(
                    pclamp(c.r, 0., 1.),
                    pclamp(c.g, 0., 1.),
                    pclamp(c.b, 0., 1.),
                    1.,
                )
                .unwrap()
            })
        }
    }
    impl From<Color> for Srgb32 {
        fn from(c: Color) -> Srgb32 {
            Srgb32::new(c.red(), c.green(), c.blue())
        }
    }

    impl From<Srgba32> for Color {
        fn from(c: Srgba32) -> Color {
            Color::from_rgba(c.r, c.g, c.b, 1.).unwrap_or_else(|| {
                Color::from_rgba(
                    pclamp(c.r, 0., 1.),
                    pclamp(c.g, 0., 1.),
                    pclamp(c.b, 0., 1.),
                    pclamp(c.a, 0., 1.),
                )
                .unwrap()
            })
        }
    }
    impl From<Color> for Srgba32 {
        fn from(c: Color) -> Srgba32 {
            Srgba32::new(c.red(), c.green(), c.blue(), c.alpha())
        }
    }

    // f32 (premultiplied)

    impl From<Srgb32> for PmColor {
        fn from(c: Srgb32) -> PmColor {
            Color::from(c).premultiply()
        }
    }
    impl From<PmColor> for Srgb32 {
        fn from(c: PmColor) -> Srgb32 {
            c.demultiply().into()
        }
    }

    impl From<Srgba32> for PmColor {
        fn from(c: Srgba32) -> PmColor {
            Color::from(c).premultiply()
        }
    }
    impl From<PmColor> for Srgba32 {
        fn from(c: PmColor) -> Srgba32 {
            c.demultiply().into()
        }
    }
}

#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
mod notcurses {
    use crate::{Srgb8, Srgba8};
    use notcurses::{Rgb, Rgba};

    impl Srgba8 {
        pub fn to_notcurses(&self) -> Rgba {
            self.into()
        }
    }

    impl From<Srgb8> for Rgb {
        fn from(c: Srgb8) -> Rgb {
            Rgb::new(c.r, c.g, c.b)
        }
    }
    impl From<Rgb> for Srgb8 {
        fn from(c: Rgb) -> Srgb8 {
            let (r, g, b) = c.into();
            Srgb8::new(r, g, b)
        }
    }

    impl From<Srgba8> for Rgba {
        fn from(c: Srgba8) -> Rgba {
            Rgba::new(c.r, c.g, c.b, c.a)
        }
    }
    impl From<Rgba> for Srgba8 {
        fn from(c: Rgba) -> Srgba8 {
            let (r, g, b, a) = c.into();
            Srgba8::new(r, g, b, a)
        }
    }
    impl From<&Srgba8> for Rgba {
        fn from(c: &Srgba8) -> Rgba {
            Rgba::new(c.r, c.g, c.b, c.a)
        }
    }
}

#[cfg(feature = "approx")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "approx")))]
mod impl_approx {
    use crate::{Color, LinearSrgb32, LinearSrgba32, Oklab32, Oklch32, Srgb32, Srgba32};
    use approx::{AbsDiffEq, RelativeEq, UlpsEq};

    // MAYBE add generic versions. E.g. `fn abs<T>(n: T)`.
    #[cfg(not(feature = "std"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    #[inline(always)]
    fn abs(n: f32) -> f32 {
        libm::fabsf(n)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    #[inline(always)]
    fn abs(n: f32) -> f32 {
        f32::abs(n)
    }

    #[cfg(not(feature = "std"))]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    #[inline(always)]
    fn signum(n: f32) -> f32 {
        libm::copysignf(n, 1.0)
    }
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    #[inline(always)]
    fn signum(n: f32) -> f32 {
        f32::signum(n)
    }

    // Implements approx traits
    //
    // # Args
    // * $T: the color type (e.g. Srgb32)
    // * $t: the type of each component (e.g. f32)
    //
    // MAYBE divide in two versions, one with just 3 components, for performance.
    macro_rules! impl_approx {
        // implements for a series of color types with the same component type
        (all $t:ty: $( $T:ty ),+) => {
            $( impl_approx![$T, $t]; )+
        };

        // implements for a single color type
        ($T:ty, $t:ty) => {
            impl AbsDiffEq for $T {
                type Epsilon = $t;
                fn default_epsilon() -> $t {
                    <$t>::EPSILON
                }
                fn abs_diff_eq(&self, other: &Self, epsilon: $t) -> bool {
                    let s: [$t; 4] = self.color_to_array4();
                    let o: [$t; 4] = other.color_to_array4();

                    abs(s[0] - o[0]) <= epsilon
                        && abs(s[1] - o[1]) <= epsilon
                        && abs(s[2] - o[2]) <= epsilon
                        && abs(s[3] - o[3]) <= epsilon
                }
            }
            impl RelativeEq for $T {
                fn default_max_relative() -> $t {
                    <$t>::EPSILON
                }
                fn relative_eq(&self, other: &Self, epsilon: $t, max_relative: $t) -> bool {
                    let s: [$t; 4] = self.color_to_array4();
                    let o: [$t; 4] = other.color_to_array4();

                    // Handle same infinities
                    if s[0] == o[0] && s[1] == o[1] && s[2] == o[2] && s[3] == o[3] {
                        return true;
                    }

                    // Handle remaining infinities
                    if s[0].is_infinite()
                        || o[0].is_infinite()
                        || s[1].is_infinite()
                        || o[1].is_infinite()
                        || s[2].is_infinite()
                        || o[2].is_infinite()
                        || s[3].is_infinite()
                        || o[3].is_infinite()
                    {
                        return false;
                    }

                    let abs_diff_0 = abs(s[0] - o[0]);
                    let abs_diff_1 = abs(s[1] - o[1]);
                    let abs_diff_2 = abs(s[2] - o[2]);
                    let abs_diff_3 = abs(s[3] - o[3]);

                    // For when the numbers are really close together
                    if abs_diff_0 <= epsilon
                        && abs_diff_1 <= epsilon
                        && abs_diff_2 <= epsilon
                        && abs_diff_3 <= epsilon
                    {
                        return true;
                    }

                    let abs_self_0 = abs(s[0]);
                    let abs_self_1 = abs(s[1]);
                    let abs_self_2 = abs(s[2]);
                    let abs_self_3 = abs(s[3]);

                    let abs_other_0 = abs(o[0]);
                    let abs_other_1 = abs(o[1]);
                    let abs_other_2 = abs(o[2]);
                    let abs_other_3 = abs(o[3]);

                    let largest_0 = if abs_other_0 > abs_self_0 {
                        abs_other_0
                    } else {
                        abs_self_0
                    };
                    let largest_1 = if abs_other_1 > abs_self_1 {
                        abs_other_1
                    } else {
                        abs_self_1
                    };
                    let largest_2 = if abs_other_2 > abs_self_2 {
                        abs_other_2
                    } else {
                        abs_self_2
                    };
                let largest_3 = if abs_other_3 > abs_self_3 {
                        abs_other_3
                    } else {
                        abs_self_3
                    };

                    // Use a relative difference comparison
                    abs_diff_0 <= largest_0 * max_relative
                        && abs_diff_1 <= largest_1 * max_relative
                        && abs_diff_2 <= largest_2 * max_relative
                        && abs_diff_3 <= largest_3 * max_relative
                }
            }

            impl UlpsEq for $T {
                fn default_max_ulps() -> u32 {
                    4
                }

                fn ulps_eq(&self, other: &Self, epsilon: $t, max_ulps: u32) -> bool {
                    let s: [$t; 4] = self.color_to_array4();
                    let o: [$t; 4] = other.color_to_array4();

                    // For when the numbers are really close together
                    if self.abs_diff_eq(other, epsilon) {
                        return true;
                    }

                    // Trivial negative sign check
                    if signum(s[0]) != signum(o[0])
                        && signum(s[1]) != signum(o[1])
                        && signum(s[2]) != signum(o[2])
                        && signum(s[3]) != signum(o[3])
                    {
                        return false;
                    }

                    // ULPS difference comparison
                    let int_self_0: u32 = s[0].to_bits();
                    let int_self_1: u32 = s[1].to_bits();
                    let int_self_2: u32 = s[2].to_bits();
                    let int_self_3: u32 = s[3].to_bits();
                    let int_other_0: u32 = o[0].to_bits();
                    let int_other_1: u32 = o[1].to_bits();
                    let int_other_2: u32 = o[2].to_bits();
                    let int_other_3: u32 = o[3].to_bits();

                    int_self_0.abs_diff(int_other_0) <= max_ulps
                        && int_self_1.abs_diff(int_other_1) <= max_ulps
                        && int_self_2.abs_diff(int_other_2) <= max_ulps
                        && int_self_3.abs_diff(int_other_3) <= max_ulps
                }
            }
        };
    }
    impl_approx![all f32: Srgb32, Srgba32, LinearSrgb32, LinearSrgba32, Oklab32, Oklch32];
}
