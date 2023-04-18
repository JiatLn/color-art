use crate::helper::round;

/// Convert `RGB` to `YCbCr`
///
/// reference: <https://docs.opencv.org/4.7.0/de/d25/imgproc_color_conversions.html#color_convert_rgb_ycrcb>
pub fn rgb2ycbcr(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = color;
    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let cr = (r - y) * 0.713 + 128.0;
    let cb = (b - y) * 0.564 + 128.0;
    (round(y, 4), round(cb, 4), round(cr, 4))
}

/// Convert `YCbCr` to `RGB`
///
/// reference: <https://docs.opencv.org/4.7.0/de/d25/imgproc_color_conversions.html#color_convert_rgb_ycrcb>
pub fn ycbcr2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (y, cb, cr) = color;
    let r = y + 1.403 * (cr - 128.0);
    let g = y - 0.344 * (cb - 128.0) - 0.714 * (cr - 128.0);
    let b = y + 1.773 * (cb - 128.0);
    (round(r, 0), round(g, 0), round(b, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2ycbcr() {
        assert_eq!(rgb2ycbcr((255.0, 255.0, 0.0)), (225.93, 0.5755, 148.7269));
        assert_eq!(rgb2ycbcr((255.0, 0.0, 0.0)), (76.245, 84.9978, 255.4523));
    }

    #[test]
    fn test_ycbcr2rgb() {
        assert_eq!(ycbcr2rgb((225.93, 0.5755, 148.7269)), (255.0, 255.0, 0.0));
        assert_eq!(ycbcr2rgb((76.245, 84.9978, 255.4523)), (255.0, 0.0, 0.0));
    }
}
