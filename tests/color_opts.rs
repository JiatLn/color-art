use color_art::Color;
use std::str::FromStr;

#[test]
fn test_color_mix() {
    let mut color = Color::from_str("rgba(0, 127, 255, 1)").unwrap();
    let color2 = Color::from_str("yellow").unwrap();
    color.mix_with(&color2, 0.6).unwrap();
    assert_eq!(color.rgba(), "rgba(153, 204, 102, 1)");

    let mut color = Color::from_str("rgba(0, 127, 255, 1)").unwrap();
    color.tint(0.5).unwrap();
    assert_eq!(color.rgba(), "rgba(128, 191, 255, 1)");

    let mut color = Color::from_str("rgba(0, 127, 255, 1)").unwrap();
    color.shade(0.5).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 64, 128, 1)");
}

#[test]
fn test_color_darken() {
    let mut color = Color::from_str("rgba(0, 127, 255, 1)").unwrap();
    color.darken(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 102, 204, 1)");

    let mut color = Color::from_str("rgba(0, 127, 255, 1)").unwrap();
    color.lighten(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(51, 153, 255, 1)");
}

#[test]
fn test_color_fade() {
    let mut color = Color::from_str("rgba(0, 127, 255, 0.8)").unwrap();
    color.fade(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.1)");

    let mut color = Color::from_str("rgba(0, 127, 255, 0.8)").unwrap();
    color.fade_in(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.9)");

    let mut color = Color::from_str("rgba(0, 127, 255, 0.8)").unwrap();
    color.fade_out(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.7)");
}
