use crate::helper::*;

// D65 standard referent
const XN: f64 = 0.950470;
const YN: f64 = 1.0;
const ZN: f64 = 1.088830;

const T0: f64 = 4.0 / 29.0;
const T1: f64 = 6.0 / 29.0;
const T2: f64 = 3.0 * T1 * T1;
const T3: f64 = T1 * T1 * T1;

/// Convert RGB to XYZ.
pub fn rgb2xyz(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let r = rgb_xyz(r);
    let g = rgb_xyz(g);
    let b = rgb_xyz(b);

    let x = (r * 0.4124564 + g * 0.3575761 + b * 0.1804375) / XN;
    let y = (r * 0.2126729 + g * 0.7151522 + b * 0.0721750) / YN;
    let z = (r * 0.0193339 + g * 0.1191920 + b * 0.9503041) / ZN;

    let x = xyz_lab(x);
    let y = xyz_lab(y);
    let z = xyz_lab(z);

    (x, y, z)
}

fn xyz_lab(t: f64) -> f64 {
    if t > T3 {
        t.powf(1.0 / 3.0)
    } else {
        t / T2 + T0
    }
}

fn rgb_xyz(r: f64) -> f64 {
    if r <= 0.04045 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert XYZ to RGB.
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
