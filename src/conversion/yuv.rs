use crate::utils::*;

static RGB2YUV_COEFFS: [f64; 5] = [0.299, 0.587, 0.114, 0.492, 0.877];
static YUV2RGB_COEFFS: [f64; 4] = [2.032, -0.395, -0.581, 1.14];

/// Convert `RGB` to `YUV`
///
/// reference: [RGB2YUV](https://github.com/opencv/opencv_contrib/blob/master/modules/cudev/include/opencv2/cudev/functional/detail/color_cvt.hpp#L363)
pub fn rgb2yuv(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let r = color[0];
    let g = color[1];
    let b = color[2];
    let y = RGB2YUV_COEFFS[0] * r + RGB2YUV_COEFFS[1] * g + RGB2YUV_COEFFS[2] * b;
    let u = RGB2YUV_COEFFS[3] * (b - y);
    let v = RGB2YUV_COEFFS[4] * (r - y);
    vec![y, u, v]
}

/// Convert `YUV` to `RGB`
///
/// reference: [YUV2RGB](https://github.com/opencv/opencv_contrib/blob/master/modules/cudev/include/opencv2/cudev/functional/detail/color_cvt.hpp#L407)
pub fn yuv2rgb(color: &[f64]) -> Vec<f64> {
    let y = color[0];
    let u = color[1];
    let v = color[2];
    let r = y + YUV2RGB_COEFFS[3] * v;
    let g = y + YUV2RGB_COEFFS[1] * u + YUV2RGB_COEFFS[2] * v;
    let b = y + YUV2RGB_COEFFS[0] * u;
    vec![round(r * 255.0, 0), round(g * 255.0, 0), round(b * 255.0, 0)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2yuv() {
        assert_eq!(
            rgb2yuv(&[255.0, 255.0, 0.0]),
            vec![0.8859999999999999, -0.43591199999999997, 0.0999780000000001]
        );
        assert_eq!(rgb2yuv(&vec![255.0, 0.0, 0.0]), vec![0.299, -0.147108, 0.614777]);
    }

    #[test]
    fn test_yuv2rgb() {
        assert_eq!(
            yuv2rgb(&[0.8859999999999999, -0.43591199999999997, 0.0999780000000001]),
            vec![255.0, 255.0, 0.0]
        );
        assert_eq!(yuv2rgb(&[0.299, -0.147108, 0.614777]), vec![255.0, 0.0, 0.0]);
    }
}
