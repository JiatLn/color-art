use color_art::Color;
use std::str::FromStr;

#[test]
fn test_color_channel() {
    let color = Color::from_str("#abcdef").unwrap();

    assert_eq!(color.red(), 171);
    assert_eq!(color.green(), 205);
    assert_eq!(color.blue(), 239);
    assert_eq!(color.alpha(), 1.0);

    assert_eq!(color.hue(), 210.0);
    assert_eq!(color.saturation(), 0.68);
    assert_eq!(color.lightness(), 0.8);

    assert_eq!(color.hsv_hue(), 210.0);
    assert_eq!(color.hsv_saturation(), 0.28);
    assert_eq!(color.hsv_value(), 0.94);

    assert_eq!(color.luma(), 0.59);
    assert_eq!(color.luminance(), 0.79);
}
