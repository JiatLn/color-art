use color_art::Color;
use std::str::FromStr;

#[test]
fn test_color_channel() {
    let mut color = Color::from_str("rgba(0, 127, 255, 1)").unwrap();
    color.fade(0.5).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.5)");

    let mut color = Color::from_str("#007fff").unwrap();
    color.tint(0.5).unwrap();
    assert_eq!(color.rgba(), "rgba(128, 191, 255, 1)");

    let mut color = Color::from_str("#007fff").unwrap();
    color.shade(0.5).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 64, 128, 1)");
}
