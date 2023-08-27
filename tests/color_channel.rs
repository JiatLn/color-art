use color_art::color;

#[test]
fn test_color_channel() {
    let color = color!(#abcdef);

    assert_eq!(color.red(), 171);
    assert_eq!(color.green(), 205);
    assert_eq!(color.blue(), 239);
    assert_eq!(color.alpha(), 1.0);

    assert_eq!(color.hue(), 210.0);
    assert_eq!(color.saturation(), 0.6800000000000003);
    assert_eq!(color.lightness(), 0.803921568627451);
    assert_eq!(color.whiteness(), 0.6705882352941176);
    assert_eq!(color.blackness(), 0.06274509803921569);

    assert_eq!(color.hsv_hue(), 210.0);
    assert_eq!(color.hsv_saturation(), 0.2845188284518829);
    assert_eq!(color.hsv_value(), 0.9372549019607843);

    assert_eq!(color.luma(), 0.5855256521034803);
    assert_eq!(color.luminance(), 0.5855256521034803);

    assert_eq!(color.gray(), 198.71);
}
