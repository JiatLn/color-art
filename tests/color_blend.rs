use color_art::{ color, blend, BlendMode };

#[test]
fn test_color_blend() {
    let c1 = color!(#4cbbfc);
    let c2 = color!(#ee2);

    let c3 = blend(&c1, &c2, BlendMode::Multiply);
    assert_eq!(c3.hex(), "#47af22");

    let c3 = blend(&c1, &c2, BlendMode::Darken);
    assert_eq!(c3.hex(), "#4cbb22");

    let c3 = blend(&c1, &c2, BlendMode::Lighten);
    assert_eq!(c3.hex(), "#eeeefc");
}
