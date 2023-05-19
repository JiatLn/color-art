use color_art::{ distance, distance_with, color, ColorSpace };

#[test]
fn test_color_calc() {
    let color1 = color!(#abcdef);
    let color2 = color!(#123456);

    let d = distance(&color1, &color2);
    assert_eq!(d, 265.00377355803823);

    let d = distance_with(&color1, &color2, ColorSpace::HSL);
    assert_eq!(d, 0.6005697492120621);
}
