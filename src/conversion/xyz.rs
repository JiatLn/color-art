use super::utils::*;
use crate::utils::*;

/// Convert RGB to XYZ.
pub fn rgb2xyz(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let color = lin_srgb(&color);

    lin_srgb_to_xyz(&color)
}

/// Convert XYZ to RGB.
pub fn xyz2rgb(color: &[f64]) -> Vec<f64> {
    let color = xyz_to_lin_srgb(color);

    lin_srgb(&color)
        .iter()
        .map(|&v| round(v * 255.0, 0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2xyz() {
        assert_eq!(
            rgb2xyz(&[118.0, 84.0, 205.0]),
            vec![0.21659503867453317, 0.1459993720802233, 0.5943650051071222]
        );

        assert_eq!(
            rgb2xyz(&vec![255.0, 255.0, 0.0]),
            vec![0.7699751386498375, 0.9278076846392662, 0.13852559851021784]
        );

        assert_eq!(
            rgb2xyz(&[255.0, 0.0, 0.0]),
            vec![0.4123907992659595, 0.21263900587151036, 0.01933081871559185]
        );

        assert_eq!(
            rgb2xyz(&[162.0, 184.0, 255.0]),
            vec![0.5008777711244343, 0.49181501188347304, 1.0146489717861926]
        );
    }

    #[test]
    fn test_xyz2rgb() {
        assert_eq!(xyz2rgb(&[0.770033, 0.927831, 0.138527]), vec![255.0, 255.0, 0.0]);
        assert_eq!(xyz2rgb(&[0.412453, 0.212671, 0.019334]), vec![255.0, 0.0, 0.0]);
        assert_eq!(xyz2rgb(&[0.70047, 0.723315, 1.048516]), vec![92.0, 122.0, 255.0]);
    }
}
