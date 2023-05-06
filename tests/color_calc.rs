use color_art::{distance, distance_with, Color, ColorSpace};
use std::str::FromStr;

#[test]
fn test_color_calc() {
    let color1 = Color::from_str("#abcdef").unwrap();
    let color2 = Color::from_str("#123456").unwrap();

    let d = distance(&color1, &color2);
    assert_eq!(d, 265.00377355803823);

    let d = distance_with(&color1, &color2, ColorSpace::HSL);
    assert_eq!(d, 0.6005717609078869);
}
