use color_art::color;

#[test]
fn test_color_hex() {
    let color = color!(#ffffff88);
    assert_eq!(color.hex(), "#fff8");
    assert_eq!(color.hex_full(), "#ffffff88");

    let color = color!(#ff00ff);
    assert_eq!(color.hex(), "#f0f");
    assert_eq!(color.hex_full(), "#ff00ff");
}
