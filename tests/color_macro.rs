use color_art::color;

#[test]
fn test_color_marco() {
    let color = color!(rgb, 255, 255, 0);
    assert_eq!(color.rgb(), "rgb(255, 255, 0)");
}
