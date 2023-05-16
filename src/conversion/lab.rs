use super::{ utils::{ lab_xyz, xyz_rgb, XN, YN, ZN }, xyz::rgb2xyz };
use crate::utils::*;

pub fn rgb2lab(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = rgb2xyz(color);

    let l = 116.0 * y - 16.0;
    let a = 500.0 * (x - y);
    let b = 200.0 * (y - z);

    let l = if l < 0.0 { 0.0 } else { l };

    (round(l, 4), round(a, 4), round(b, 4))
}

pub fn lab2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (l, a, b) = color;

    let y = (l + 16.0) / 116.0;
    let x = y + a / 500.0;
    let z = y - b / 200.0;

    let y = YN * lab_xyz(y);
    let x = XN * lab_xyz(x);
    let z = ZN * lab_xyz(z);

    let r = 3.2406 * x - 1.5372 * y - 0.4986 * z;
    let g = -0.9689 * x + 1.8758 * y + 0.0415 * z;
    let b = 0.0557 * x - 0.204 * y + 1.057 * z;

    let r = 255.0 * xyz_rgb(r);
    let g = 255.0 * xyz_rgb(g);
    let b = 255.0 * xyz_rgb(b);

    (round(r, 0), round(g, 0), round(b, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2lab() {
        assert_eq!(rgb2lab((255.0, 255.0, 0.0)), (97.1393, -21.5537, 94.478));
        assert_eq!(rgb2lab((0.0, 255.0, 0.0)), (87.7347, -86.1827, 83.1793));
    }

    #[test]
    fn test_lab2rgb() {
        assert_eq!(lab2rgb((97.1393, -21.5537, 94.478)), (255.0, 255.0, 0.0));
        assert_eq!(lab2rgb((87.7347, -86.1827, 83.1793)), (0.0, 255.0, 0.0));
    }
}
