use color_art::color;

#[test]
fn test_color_stringify() {
    let color = color!(#ffff00);

    assert_eq!(color.hex(), "#ff0");
    assert_eq!(color.hex_full(), "#ffff00");
    assert_eq!(color.rgb(), "rgb(255, 255, 0)");
    assert_eq!(color.rgba(), "rgba(255, 255, 0, 1)");
    assert_eq!(color.hsl(), "hsl(60, 100%, 50%)");
    assert_eq!(color.hsla(), "hsla(60, 100%, 50%, 1)");
    assert_eq!(color.hsv(), "hsv(60, 100%, 100%)");
    assert_eq!(color.hsi(), "hsi(60, 100%, 66.67%)");
    assert_eq!(color.hwb(), "hwb(60, 0%, 0%)");
    assert_eq!(color.cmyk(), "cmyk(0%, 0%, 100%, 0%)");
    assert_eq!(color.name(), "yellow");
    assert_eq!(color.xyz(), "xyz(0.769975, 0.927808, 0.138526)");
    assert_eq!(color.yiq(), "yiq(0.886, 0.32126, -0.31114)");
    assert_eq!(color.yuv(), "yuv(0.886, -0.4359, 0.1)");
    assert_eq!(color.lab(), "lab(97.61, -15.75, 93.39)");
}
