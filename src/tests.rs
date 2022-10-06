// acolor::tests
//
//!
//

use crate::*;
use approx::assert_relative_eq;
use iunorm::Unorm8;

#[test]
fn min_max_clamp() {
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

#[test]
fn srgb8() {
    let c = Srgb8::new(0xA, 0xB, 0xC);

    // back and forth
    assert_eq![c.to_srgba8(255).to_srgb8(), c];
    assert_eq![c.to_srgb32().to_srgb8(), c];
    assert_eq![c.to_srgba32(1.).to_srgb8(), c];
    assert_eq![c.to_linear_srgb32().to_srgb8(), c];
    assert_eq![c.to_linear_srgba32(1.).to_srgb8(), c];
    assert_eq![c.to_oklab32().to_srgb8(), c];
    assert_eq![c.to_oklch32().to_srgb8(), c];

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
fn srgba8() {
    let c = Srgba8::new(0xA, 0xB, 0xC, 0xD);

    // back and forth
    assert_eq![c.to_srgb8().to_srgba8(0xD), c];
    assert_eq![c.to_srgb32().to_srgba8(0xD), c];
    assert_eq![c.to_srgba32().to_srgba8(), c];
    assert_eq![c.to_linear_srgb32().to_srgba8(0xD), c];
    assert_eq![c.to_linear_srgba32().to_srgba8(), c];
    assert_eq![c.to_oklab32().to_srgba8(0xD), c];
    assert_eq![c.to_oklch32().to_srgba8(0xD), c];
}

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
