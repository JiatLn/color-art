use color_art::Color;

#[test]
fn test_color_from_space() {
    // RGB
    let color = Color::from_rgb(255, 255, 0).unwrap();
    assert_eq!(color.hex(), "#ff0");

    // RGBA
    let color = Color::from_rgba(255, 255, 0, 0.5).unwrap();
    assert_eq!(color.hex(), "#ffff0080");

    // HSL
    let color = Color::from_hsl(60.0, 1.0, 0.5).unwrap();
    assert_eq!(color.hex(), "#ff0");

    // HSV
    let color = Color::from_hsv(60.0, 1.0, 1.0).unwrap();
    assert_eq!(color.hex(), "#ff0");

    // CMYK
    let color = Color::from_cmyk(0.0, 0.0, 1.0, 0.0).unwrap();
    assert_eq!(color.hex(), "#ff0");

    // HEX
    let color = Color::from_hex("#ffff00").unwrap();
    assert_eq!(color.hex(), "#ff0");

    // Name
    let color = Color::from_name("yellow").unwrap();
    assert_eq!(color.hex(), "#ff0");
}
