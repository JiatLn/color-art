use color_art::color;

#[test]
fn test_color_marco() {
    let color = color!(rgb, 255, 255, 0);
    assert_eq!(color.rgb(), "rgb(255, 255, 0)");

    let color = color!(#1890ff);
    assert_eq!(color.hex(), "#1890ff");
}
