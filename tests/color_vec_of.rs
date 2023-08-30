use color_art::{color, ColorSpace};

#[test]
fn test_color_hex() {
    let color = color!(#ff00ff);

    let vec = color.vec_of("hex");
    assert_eq!(vec, vec![255.0, 0.0, 255.0]);

    let vec = color.vec_of(ColorSpace::HEXA);
    assert_eq!(vec, vec![255.0, 0.0, 255.0, 1.0]);

    let vec = color.vec_of(ColorSpace::RGB);
    assert_eq!(vec, vec![255.0, 0.0, 255.0]);

    let vec = color.vec_of("hsl");
    assert_eq!(vec, vec![300.0, 1.0, 0.5]);
}
