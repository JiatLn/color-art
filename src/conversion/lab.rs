use super::{ utils::{ xyz2lab, d65_to_d50, lab2xyz, d50_to_d65 }, xyz::{ rgb2xyz, xyz2rgb } };

pub fn rgb2lab(color: &[f64]) -> Vec<f64> {
    let xyz = rgb2xyz(color);
    let xyz = d65_to_d50(xyz);
    xyz2lab(xyz)
}

pub fn lab2rgb(color: &[f64]) -> Vec<f64> {
    let xyz = lab2xyz(color);
    let xyz = d50_to_d65(xyz);
    xyz2rgb(&xyz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2lab() {
        assert_eq!(
            rgb2lab(&[255.0, 255.0, 0.0]),
            vec![97.60701009682252, -15.749846639252551, 93.39361164266084]
        );
        assert_eq!(
            rgb2lab(&[0.0, 255.0, 0.0]),
            vec![87.818536331152, -79.271082238548, 80.99459785152246]
        );
    }

    #[test]
    fn test_lab2rgb() {
        assert_eq!(
            lab2rgb(&[97.60701009682252, -15.749846639252551, 93.39361164266084]),
            vec![255.0, 255.0, 0.0]
        );
        assert_eq!(
            lab2rgb(&[87.818536331152, -79.271082238548, 80.99459785152246]),
            vec![0.0, 255.0, 0.0]
        );
    }
}
