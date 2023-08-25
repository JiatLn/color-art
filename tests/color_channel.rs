use color_art::color;

#[test]
fn test_color_channel() {
    let color = color!(#abcdef);

    assert_eq!(color.red(), 171);
    assert_eq!(color.green(), 205);
    assert_eq!(color.blue(), 239);
    assert_eq!(color.alpha(), 1.0);

    assert_eq!(color.hue(), 210.0);
    assert_eq!(color.saturation(), 0.68);
    assert_eq!(color.lightness(), 0.8039);
    assert_eq!(color.whiteness(), 0.6706);
    assert_eq!(color.blackness(), 0.0627);

    assert_eq!(color.hsv_hue(), 210.0);
    assert_eq!(color.hsv_saturation(), 0.2845);
    assert_eq!(color.hsv_value(), 0.9373);

    assert_eq!(color.luma(), 0.5855256521034803);
    assert_eq!(color.luminance(), 0.5855256521034803);

    assert_eq!(color.gray(), 198.71);
}
