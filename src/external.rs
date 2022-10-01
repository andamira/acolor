// acolor::external
//
//! Optional conversions to external types.
//

#[cfg(feature = "macroquad")]
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

#[cfg(feature = "tiny-skia")]
mod tiny_skia {
    use crate::clamp;
    use crate::{Srgb32, Srgb8, Srgba32, Srgba8};
    use tiny_skia::{Color, ColorU8};
    // use tiny_skia::{PremultipliedColorU8, PremultipliedColorU8};

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

    // f32

    impl From<Srgb32> for Color {
        fn from(c: Srgb32) -> Color {
            Color::from_rgba(c.r, c.g, c.b, 1.).unwrap_or_else(|| {
                Color::from_rgba(
                    clamp(c.r, 0., 1.),
                    clamp(c.g, 0., 1.),
                    clamp(c.b, 0., 1.),
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
                    clamp(c.r, 0., 1.),
                    clamp(c.g, 0., 1.),
                    clamp(c.b, 0., 1.),
                    clamp(c.a, 0., 1.),
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
}

#[cfg(feature = "notcurses")]
mod notcurses {
    use crate::{Srgb8, Srgba8};
    use notcurses::{Rgb, Rgba};

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
}
