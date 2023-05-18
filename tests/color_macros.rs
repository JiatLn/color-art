use color_art::color;

#[test]
fn test_color_marcos() {
    let color = color!(rgb, 255, 255, 0);
    assert_eq!(color.rgb(), "rgb(255, 255, 0)");

    let color = color!(#1890ff);
    assert_eq!(color.hex(), "#1890ff");

    let color = color!(#1890ff33);
    assert_eq!(color.hex(), "#1890ff33");

    let color = color!(rgb(255, 255, 0));
    assert_eq!(color.rgb(), "rgb(255, 255, 0)");

    let color = color!(rgba(255, 255, 0, 0.5));
    assert_eq!(color.rgba(), "rgba(255, 255, 0, 0.5)");
}
