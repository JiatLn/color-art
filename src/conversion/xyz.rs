use crate::helper::*;

/// Convert RGB to XYZ.
#[allow(dead_code)]
pub fn rgb2xyz(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    dbg!((r, g, b));

    let x = 0.412453 * r + 0.357580 * g + 0.180423 * b;
    let y = 0.212671 * r + 0.715160 * g + 0.072169 * b;
    let z = 0.019334 * r + 0.119193 * g + 0.950227 * b;

    dbg!((x, y, z));

    (round(x, 6), round(y, 6), round(z, 6))
}

/// Convert XYZ to RGB.
#[allow(dead_code)]
pub fn xyz2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (x, y, z) = color;
    let r = 3.2406 * x - 1.5372 * y - 0.4986 * z;
    let g = -0.9689 * x + 1.8758 * y + 0.0415 * z;
    let b = 0.0557 * x - 0.2040 * y + 1.0570 * z;
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
        assert_eq!(rgb2xyz((255.0, 0.0, 0.0)), (0.412453, 0.212671, 0.019334));
        assert_eq!(
            rgb2xyz((162.0, 184.0, 255.0)),
            (0.70047, 0.723315, 1.048516)
        );
    }

    #[test]
    fn test_xyz2rgb() {
        assert_eq!(xyz2rgb((0.412453, 0.212671, 0.019334)), (255.0, 0.0, 0.0));
        assert_eq!(
            xyz2rgb((0.70047, 0.723315, 1.048516)),
            (162.0, 184.0, 255.0)
        );
    }
}
