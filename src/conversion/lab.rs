use super::{ utils::{ xyz2lab, d65_to_d50, lab2xyz, d50_to_d65 }, xyz::{ rgb2xyz, xyz2rgb } };

pub fn rgb2lab(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = rgb2xyz(color);

    let xyz = vec![x, y, z];

    let xyz = d65_to_d50(xyz);

    let lab = xyz2lab(xyz);

    (lab[0], lab[1], lab[2])
}

pub fn lab2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (l, a, b) = color;

    let xyz = lab2xyz(vec![l, a, b]);

    let xyz = d50_to_d65(xyz);

    let xyz = (xyz[0], xyz[1], xyz[2]);

    xyz2rgb(xyz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2lab() {
        assert_eq!(
            rgb2lab((255.0, 255.0, 0.0)),
            (97.60701009682252, -15.749846639252551, 93.39361164266084)
        );
        assert_eq!(
            rgb2lab((0.0, 255.0, 0.0)),
            (87.818536331152, -79.271082238548, 80.99459785152246)
        );
    }

    #[test]
    fn test_lab2rgb() {
        assert_eq!(
            lab2rgb((97.60701009682252, -15.749846639252551, 93.39361164266084)),
            (255.0, 255.0, 0.0)
        );
        assert_eq!(
            lab2rgb((87.818536331152, -79.271082238548, 80.99459785152246)),
            (0.0, 255.0, 0.0)
        );
    }
}
