// acolor::tests
//
//!
//

use crate::*;
use iunorm::Unorm8;

#[test]
#[cfg(any(feature = "std", feature = "no-std"))]
fn srgb8() {
    let c = Srgb8::new(0xA, 0xB, 0xC);

    // convert back and forth
    assert_eq![c.to_linear_srgb32().to_srgb8(), c];
    assert_eq![c.to_linear_srgba32(1.).to_srgb8(), c];
    assert_eq![c.to_oklab32().to_srgb8(), c];
    assert_eq![c.to_oklch32().to_srgb8(), c];
}

#[test]
fn srgb8_no_default() {
    let c = Srgb8::new(0xA, 0xB, 0xC);

    // convert back and forth
    assert_eq![c.to_srgba8(255).to_srgb8(), c];
    assert_eq![c.to_srgb32().to_srgb8(), c];
    assert_eq![c.to_srgba32(1.).to_srgb8(), c];

    // check data
    assert_eq![c.to_srgba8(0xD), Srgba8::new(0xA, 0xB, 0xC, 0xD)];
    assert_eq![
        c.to_srgb32(),
        Srgb32::new(
            Unorm8(0xA).to_f32(),
            Unorm8(0xB).to_f32(),
            Unorm8(0xC).to_f32()
        )
    ];
    assert_eq![
        c.to_srgba32(0.7),
        Srgba32::new(
            Unorm8(0xA).to_f32(),
            Unorm8(0xB).to_f32(),
            Unorm8(0xC).to_f32(),
            0.7
        )
    ];
}

#[test]
#[cfg(any(feature = "std", feature = "no-std"))]
fn srgba8() {
    let c = Srgba8::new(0xA, 0xB, 0xC, 0xD);

    // convert back and forth
    assert_eq![c.to_linear_srgb32().to_srgba8(0xD), c];
    assert_eq![c.to_linear_srgba32().to_srgba8(), c];
    assert_eq![c.to_oklab32().to_srgba8(0xD), c];
    assert_eq![c.to_oklch32().to_srgba8(0xD), c];
}

#[test]
fn srgba8_no_default() {
    let c = Srgba8::new(0xA, 0xB, 0xC, 0xD);

    // convert back and forth
    assert_eq![c.to_srgb8().to_srgba8(0xD), c];
    assert_eq![c.to_srgb32().to_srgba8(0xD), c];
    assert_eq![c.to_srgba32().to_srgba8(), c];
}

#[cfg(feature = "approx")]
mod approx_tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn srgb32() {
        let c = Srgb32::new(0.1, 0.2, 0.3);

        // back and forth
        assert_relative_eq![c.to_srgb8().to_srgb32(), c, max_relative = 0.02];
        assert_relative_eq![c.to_srgba8(255).to_srgb32(), c, max_relative = 0.02];
        assert_eq![c.to_srgba32(1.).to_srgb32(), c];
        assert_relative_eq![c.to_linear_srgb32().to_srgb32(), c];
        assert_relative_eq![c.to_linear_srgba32(1.).to_srgb32(), c];
        assert_relative_eq![c.to_oklab32().to_srgb32(), c];
        assert_relative_eq![c.to_oklch32().to_srgb32(), c];
    }

    #[test]
    fn srgba32() {
        let c = Srgba32::new(0.1, 0.2, 0.3, 0.4);

        // back and forth
        assert_relative_eq![c.to_srgb8().to_srgba32(0.4), c, max_relative = 0.02];
        assert_relative_eq![c.to_srgba8().to_srgba32(), c, max_relative = 0.02];
        assert_eq![c.to_srgb32().to_srgba32(0.4), c];
        assert_relative_eq![c.to_linear_srgb32().to_srgba32(0.4), c];
        assert_relative_eq![c.to_linear_srgba32().to_srgba32(), c];
        assert_relative_eq![c.to_oklab32().to_srgba32(0.4), c];
        assert_relative_eq![c.to_oklch32().to_srgba32(0.4), c];
    }

    #[test]
    fn linear_srgb32() {
        let c = LinearSrgb32::new(0.1, 0.2, 0.3);

        // back and forth
        assert_relative_eq![c.to_srgb8().to_linear_srgb32(), c, max_relative = 8e-3];
        assert_relative_eq![c.to_srgba8(255).to_linear_srgb32(), c, max_relative = 8e-3];
        assert_relative_eq![c.to_srgb32().to_linear_srgb32(), c];
        assert_eq![c.to_linear_srgba32(1.).to_linear_srgb32(), c];
        assert_relative_eq![c.to_oklab32().to_linear_srgb32(), c];
        assert_relative_eq![c.to_oklch32().to_linear_srgb32(), c];
    }

    #[test]
    fn linear_srgba32() {
        let c = LinearSrgba32::new(0.1, 0.2, 0.3, 0.4);

        // back and forth
        assert_relative_eq![c.to_srgb8().to_linear_srgba32(0.4), c, max_relative = 8e-3];
        assert_relative_eq![c.to_srgba8().to_linear_srgba32(), c, max_relative = 8e-3];
        assert_relative_eq![c.to_srgb32().to_linear_srgba32(0.4), c];
        assert_eq![c.to_linear_srgb32().to_linear_srgba32(0.4), c];
        assert_relative_eq![c.to_oklab32().to_linear_srgba32(0.4), c];
        assert_relative_eq![c.to_oklch32().to_linear_srgba32(0.4), c];
    }

    #[test]
    fn oklab32() {
        let c = Oklab32::new(0.7, -0.1, 0.1);

        // back and forth
        assert_relative_eq![c.to_srgb8().to_oklab32(), c, max_relative = 3e-3];
        assert_relative_eq![c.to_srgba8(255).to_oklab32(), c, max_relative = 3e-3];
        assert_relative_eq![c.to_srgb32().to_oklab32(), c, max_relative = 3e-6];
        assert_relative_eq![c.to_srgba32(1.).to_oklab32(), c, max_relative = 3e-6];
        assert_relative_eq![c.to_linear_srgb32().to_oklab32(), c, max_relative = 3e-6];
        assert_relative_eq![c.to_linear_srgba32(1.).to_oklab32(), c, max_relative = 3e-6];
        assert_relative_eq![c.to_oklch32().to_oklab32(), c];
    }

    #[test]
    fn oklch32() {
        let c = Oklch32::new(0.7, 0.15, 2.5);

        // back and forth
        assert_relative_eq![c.to_srgb8().to_oklch32(), c, max_relative = 0.1];
        assert_relative_eq![c.to_srgba8(255).to_oklch32(), c, max_relative = 0.1];
        assert_relative_eq![c.to_srgb32().to_oklch32(), c, max_relative = 2e-5];
        assert_relative_eq![c.to_srgba32(1.).to_oklch32(), c, max_relative = 2e-5];
        assert_relative_eq![c.to_linear_srgb32().to_oklch32(), c, max_relative = 3e-5];
        assert_relative_eq![c.to_linear_srgba32(1.).to_oklch32(), c, max_relative = 3e-5];
        assert_relative_eq![c.to_oklab32().to_oklch32(), c];
    }
}
