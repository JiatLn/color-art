use color_art::color;

#[test]
fn test_color_mix() {
    let mut color = color!(rgb(0, 127, 255));
    let color2 = color!(rgb(255, 255, 0));
    color.mix_with(&color2, 0.6).unwrap();
    assert_eq!(color.rgb(), "rgb(153, 204, 102)");

    let mut color = color!(rgb(0, 127, 255));
    color.tint(0.5).unwrap();
    assert_eq!(color.rgb(), "rgb(128, 191, 255)");

    let mut color = color!(rgb(0, 127, 255));
    color.shade(0.5).unwrap();
    assert_eq!(color.rgb(), "rgb(0, 64, 128)");
}

#[test]
fn test_color_darken() {
    let mut color = color!(rgb(0, 127, 255));
    color.darken(0.1).unwrap();
    assert_eq!(color.rgb(), "rgb(0, 102, 204)");

    let mut color = color!(rgb(0, 127, 255));
    color.lighten(0.1).unwrap();
    assert_eq!(color.rgb(), "rgb(51, 153, 255)");
}

#[test]
fn test_color_fade() {
    let mut color = color!(rgba(0, 127, 255, 0.8));
    color.fade(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.1)");

    let mut color = color!(rgba(0, 127, 255, 0.8));
    color.fade_in(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.9)");

    let mut color = color!(rgba(0, 127, 255, 0.8));
    color.fade_out(0.1).unwrap();
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.7)");
}
