use color_art::Color;

#[test]
fn test_color_from_space() {
    // RGB
    let color = Color::from_rgb(255.0, 255.0, 0.0).unwrap();
    assert_eq!(color.hex(), "#ffff00");

    // RGBA
    let color = Color::from_rgba(255.0, 255.0, 0.0, 0.5).unwrap();
    assert_eq!(color.hex(), "#ffff00");

    // HSL
    let color = Color::from_hsl(60.0, 1.0, 0.5).unwrap();
    assert_eq!(color.hex(), "#ffff00");

    // HSV
    let color = Color::from_hsv(60.0, 1.0, 1.0).unwrap();
    assert_eq!(color.hex(), "#ffff00");

    // CMYK
    let color = Color::from_cmyk(0.0, 0.0, 1.0, 0.0).unwrap();
    assert_eq!(color.hex(), "#ffff00");

    // HEX
    let color = Color::from_hex("#ffff00").unwrap();
    assert_eq!(color.hex(), "#ffff00");
}
