use color_art::color;

#[test]
fn test_color_mix() {
    let color1 = color!(rgb(0, 127, 255));
    let color2 = color!(rgb(255, 255, 0));
    let color3 = color1.mix_with(&color2, 0.6);
    assert_eq!(color3.rgb(), "rgb(153, 204, 102)");

    let color = color!(rgb(0, 127, 255));
    let color = color.tint(0.5);
    assert_eq!(color.rgb(), "rgb(128, 191, 255)");

    let color = color!(rgb(0, 127, 255));
    let color = color.shade(0.5);
    assert_eq!(color.rgb(), "rgb(0, 64, 128)");
}

#[test]
fn test_color_darken() {
    let color = color!(rgb(0, 127, 255));
    let color = color.darken(0.1);
    assert_eq!(color.rgb(), "rgb(0, 102, 204)");

    let color = color!(rgb(0, 127, 255));
    let color = color.lighten(0.1);
    assert_eq!(color.rgb(), "rgb(51, 153, 255)");

    let color = color!(rgb(41, 121, 255));
    let color = color.darken(0.1);
    assert_eq!(color.rgb(), "rgb(0, 92, 245)");
}

#[test]
fn test_color_fade() {
    let color = color!(rgba(0, 127, 255, 0.8));
    let color = color.fade(0.1);
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.1)");

    let color = color!(rgba(0, 127, 255, 0.8));
    let color = color.fade_in(0.1);
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.9)");

    let color = color!(rgba(0, 127, 255, 0.8));
    let color = color.fade_out(0.1);
    assert_eq!(color.rgba(), "rgba(0, 127, 255, 0.7)");
}
