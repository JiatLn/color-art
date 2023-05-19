use crate::utils::*;

static RGB2YCRCB_COEFFS: [f64; 5] = [0.299, 0.587, 0.114, 0.713, 0.564];
static YCRCB2RGB_COEFFS: [f64; 4] = [1.403, -0.714, -0.344, 1.773];

/// Convert `RGB` to `YCbCr`
///
/// reference: [RGB2YCrCb](https://github.com/opencv/opencv_contrib/blob/master/modules/cudev/include/opencv2/cudev/functional/detail/color_cvt.hpp#L427)
pub fn rgb2ycbcr(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = color;
    let y = RGB2YCRCB_COEFFS[0] * r + RGB2YCRCB_COEFFS[1] * g + RGB2YCRCB_COEFFS[2] * b;
    let cr = (r - y) * RGB2YCRCB_COEFFS[3] + 128.0;
    let cb = (b - y) * RGB2YCRCB_COEFFS[4] + 128.0;
    (y, cb, cr)
}

/// Convert `YCbCr` to `RGB`
///
/// reference: [YCrCb2RGB](https://github.com/opencv/opencv_contrib/blob/master/modules/cudev/include/opencv2/cudev/functional/detail/color_cvt.hpp#L481)
pub fn ycbcr2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (y, cb, cr) = color;
    let r = y + YCRCB2RGB_COEFFS[0] * (cr - 128.0);
    let g = y + YCRCB2RGB_COEFFS[2] * (cb - 128.0) + YCRCB2RGB_COEFFS[1] * (cr - 128.0);
    let b = y + YCRCB2RGB_COEFFS[3] * (cb - 128.0);
    (round(r, 0), round(g, 0), round(b, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2ycbcr() {
        assert_eq!(rgb2ycbcr((255.0, 255.0, 0.0)), (225.93, 0.5754800000000131, 148.72691));
        assert_eq!(
            rgb2ycbcr((255.0, 0.0, 0.0)),
            (76.24499999999999, 84.99782000000002, 255.452315)
        );
    }

    #[test]
    fn test_ycbcr2rgb() {
        assert_eq!(ycbcr2rgb((225.93, 0.5754800000000131, 148.72691)), (255.0, 255.0, 0.0));
        assert_eq!(
            ycbcr2rgb((76.24499999999999, 84.99782000000002, 255.452315)),
            (255.0, 0.0, 0.0)
        );
    }
}
