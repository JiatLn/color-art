use super::utils::{rgb_xyz, xyz_lab, XN, YN, ZN};
use crate::helper::*;

static RGB2XYZ_COEFFS: [f64; 9] = [
    0.4124564, 0.3575761, 0.1804375, 0.2126729, 0.7151522, 0.0721750, 0.0193339, 0.1191920,
    0.9503041,
];

static XYZ2RGB_COEFFS: [f64; 9] = [
    3.2406, -1.5372, -0.4986, -0.9689, 1.8758, 0.0415, 0.0557, -0.2040, 1.0570,
];

/// Convert RGB to XYZ.
pub fn rgb2xyz(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let r = rgb_xyz(r);
    let g = rgb_xyz(g);
    let b = rgb_xyz(b);

    let x = (r * RGB2XYZ_COEFFS[0] + g * RGB2XYZ_COEFFS[1] + b * RGB2XYZ_COEFFS[2]) / XN;
    let y = (r * RGB2XYZ_COEFFS[3] + g * RGB2XYZ_COEFFS[4] + b * RGB2XYZ_COEFFS[5]) / YN;
    let z = (r * RGB2XYZ_COEFFS[6] + g * RGB2XYZ_COEFFS[7] + b * RGB2XYZ_COEFFS[8]) / ZN;

    let x = xyz_lab(x);
    let y = xyz_lab(y);
    let z = xyz_lab(z);

    (x, y, z)
}

/// Convert XYZ to RGB.
pub fn xyz2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = color;
    let r = XYZ2RGB_COEFFS[0] * x + XYZ2RGB_COEFFS[1] * y + XYZ2RGB_COEFFS[2] * z;
    let g = XYZ2RGB_COEFFS[3] * x + XYZ2RGB_COEFFS[4] * y + XYZ2RGB_COEFFS[5] * z;
    let b = XYZ2RGB_COEFFS[6] * x + XYZ2RGB_COEFFS[7] * y + XYZ2RGB_COEFFS[8] * z;
    (
        round(r * 255.0, 0),
        round(g * 255.0, 0),
        round(b * 255.0, 0),
    )
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
        assert_eq!(
            xyz2rgb((0.70047, 0.723315, 1.048516)),
            (162.0, 184.0, 255.0)
        );
    }
}
