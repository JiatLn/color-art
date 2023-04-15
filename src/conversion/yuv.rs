use crate::helper::{normalize_color, round};

pub fn rgb2yuv(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let u = 0.493 * (b - y);
    let v = 0.877 * (r - y);
    (round(y, 4), round(u, 4), round(v, 4))
}

pub fn yuv2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (y, u, v) = color;
    let r = y + 1.13983 * v;
    let g = y - 0.39465 * u - 0.58060 * v;
    let b = y + 2.03211 * u;
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
    fn test_rgb2yuv() {
        assert_eq!(rgb2yuv((255.0, 255.0, 0.0)), (0.886, -0.4368, 0.1));
        assert_eq!(rgb2yuv((255.0, 0.0, 0.0)), (0.299, -0.1474, 0.6148));
    }

    #[test]
    fn test_yuv2rgb() {
        assert_eq!(yuv2rgb((0.886, -0.4368, 0.1)), (255.0, 255.0, 0.0));
        assert_eq!(yuv2rgb((0.299, -0.1474, 0.6148)), (255.0, 0.0, 0.0));
    }
}
