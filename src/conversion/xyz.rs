use super::utils::{ rgb_xyz, xyz_lab, XN, YN, ZN };
use crate::utils::*;

static RGB2XYZ_MATRIX: [[f64; 3]; 3] = [
    [0.4124564, 0.3575761, 0.1804375],
    [0.2126729, 0.7151522, 0.072175],
    [0.0193339, 0.119192, 0.9503041],
];

static XYZ2RGB_MATRIX: [[f64; 3]; 3] = [
    [3.2406, -1.5372, -0.4986],
    [-0.9689, 1.8758, 0.0415],
    [0.0557, -0.204, 1.057],
];

/// Convert RGB to XYZ.
pub fn rgb2xyz(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let r = rgb_xyz(r);
    let g = rgb_xyz(g);
    let b = rgb_xyz(b);

    let rgb2xyz_matrix = RGB2XYZ_MATRIX.map(|v| v.to_vec()).to_vec();
    let xyz = multiply_matrices(rgb2xyz_matrix, vec![vec![r], vec![g], vec![b]]);

    let x = xyz[0][0] / XN;
    let y = xyz[1][0] / YN;
    let z = xyz[2][0] / ZN;

    let x = xyz_lab(x);
    let y = xyz_lab(y);
    let z = xyz_lab(z);

    (x, y, z)
}

/// Convert XYZ to RGB.
pub fn xyz2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = color;

    let xyz2rgb_matrix: Matrix = XYZ2RGB_MATRIX.map(|v| v.to_vec()).to_vec();
    let rgb = multiply_matrices(xyz2rgb_matrix, vec![vec![x], vec![y], vec![z]]);

    let r = rgb[0][0];
    let g = rgb[1][0];
    let b = rgb[2][0];

    (round(r * 255.0, 0), round(g * 255.0, 0), round(b * 255.0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2xyz() {
        assert_eq!(
            rgb2xyz((255.0, 255.0, 0.0)),
            (0.9322310141216105, 0.9753385105543646, 0.5029486352860133)
        );
        assert_eq!(
            rgb2xyz((255.0, 0.0, 0.0)),
            (0.757088316962712, 0.5969033977698898, 0.26088741519062475)
        );
        assert_eq!(
            rgb2xyz((162.0, 184.0, 255.0)),
            (0.8077140654027584, 0.7893387855629574, 0.9766808553677133)
        );
    }

    #[test]
    fn test_xyz2rgb() {
        assert_eq!(xyz2rgb((0.770033, 0.927831, 0.138527)), (255.0, 255.0, 0.0));
        assert_eq!(xyz2rgb((0.412453, 0.212671, 0.019334)), (255.0, 0.0, 0.0));
        assert_eq!(xyz2rgb((0.70047, 0.723315, 1.048516)), (162.0, 184.0, 255.0));
    }
}
