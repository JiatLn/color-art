use crate::helper::{ normalize_color, round };

static RGB2YUV_COEFFS: [f64; 5] = [0.299, 0.587, 0.114, 0.492, 0.877];
static YUV2RGB_COEFFS: [f64; 4] = [2.032, -0.395, -0.581, 1.14];

/// Convert `RGB` to `YUV`
///
/// reference: [RGB2YUV](https://github.com/opencv/opencv_contrib/blob/master/modules/cudev/include/opencv2/cudev/functional/detail/color_cvt.hpp#L363)
pub fn rgb2yuv(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);
    let y = RGB2YUV_COEFFS[0] * r + RGB2YUV_COEFFS[1] * g + RGB2YUV_COEFFS[2] * b;
    let u = RGB2YUV_COEFFS[3] * (b - y);
    let v = RGB2YUV_COEFFS[4] * (r - y);
    (round(y, 4), round(u, 4), round(v, 4))
}

/// Convert `YUV` to `RGB`
///
/// reference: [YUV2RGB](https://github.com/opencv/opencv_contrib/blob/master/modules/cudev/include/opencv2/cudev/functional/detail/color_cvt.hpp#L407)
pub fn yuv2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (y, u, v) = color;
    let r = y + YUV2RGB_COEFFS[3] * v;
    let g = y + YUV2RGB_COEFFS[1] * u + YUV2RGB_COEFFS[2] * v;
    let b = y + YUV2RGB_COEFFS[0] * u;
    (round(r * 255.0, 0), round(g * 255.0, 0), round(b * 255.0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2yuv() {
        assert_eq!(rgb2yuv((255.0, 255.0, 0.0)), (0.886, -0.4359, 0.1));
        assert_eq!(rgb2yuv((255.0, 0.0, 0.0)), (0.299, -0.1471, 0.6148));
    }

    #[test]
    fn test_yuv2rgb() {
        assert_eq!(yuv2rgb((0.886, -0.4359, 0.1)), (255.0, 255.0, 0.0));
        assert_eq!(yuv2rgb((0.299, -0.1471, 0.6148)), (255.0, 0.0, 0.0));
    }
}
