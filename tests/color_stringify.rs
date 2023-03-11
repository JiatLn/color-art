use color_art::Color;
use std::str::FromStr;

#[test]
fn test_color_stringify() {
    let color = Color::from_str("#ffff00").unwrap();

    assert_eq!(color.hex(), "#ffff00");
    assert_eq!(color.rgb(), "rgb(255, 255, 0)");
    assert_eq!(color.rgba(), "rgba(255, 255, 0, 1)");
    assert_eq!(color.hsl(), "hsl(60, 100%, 50%)");
    assert_eq!(color.hsv(), "hsv(60, 100%, 100%)");
    assert_eq!(color.hwb(), "hwb(60, 0%, 0%)");
    assert_eq!(color.cmyk(), "cmyk(0%, 0%, 100%, 0%)");
    assert_eq!(color.name(), "yellow");
    assert_eq!(color.xyz(), "xyz(0.932231, 0.975339, 0.502949)");
}
