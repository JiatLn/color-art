use crate::conversion::hsl;
use crate::utils::*;

/// Convert RGB to HWB.
///
/// Reference from [Converting sRGB Colors to HWB](https://w3c.github.io/csswg-drafts/css-color/#rgb-to-hwb)
pub fn rgb2hwb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let hsl = hsl::rgb2hsl(color);

    let hue = hsl.0;
    let whiteness = min;
    let blackness = 1.0 - max;

    (hue, whiteness, blackness)
}

/// Reference from [Converting HWB Colors to sRGB](https://w3c.github.io/csswg-drafts/css-color/#hsl-to-rgb)
pub fn hwb2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (hue, whiteness, blackness) = color;
    if whiteness + blackness >= 1.0 {
        let gray = whiteness / (whiteness + blackness);
        return (gray, gray, gray);
    }
    let rgb = hsl::hsl2rgb((hue, 1.0, 0.5));
    let mut rgb_vec = vec![rgb.0, rgb.1, rgb.2];
    for i in 0..3 {
        rgb_vec[i] *= 1.0 - whiteness - blackness;
        rgb_vec[i] += whiteness * 255.0;
        rgb_vec[i] = round(rgb_vec[i], 0);
    }
    vec2tuple(rgb_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hwb() {
        assert_eq!(rgb2hwb((0.0, 255.0, 102.0)), (144.0, 0.0, 0.0));
        assert_eq!(
            rgb2hwb((86.0, 59.0, 133.0)),
            (261.8918918918919, 0.23137254901960785, 0.4784313725490196)
        );
    }

    #[test]
    fn test_hwb2rgb() {
        assert_eq!(hwb2rgb((144.0, 0.0, 0.0)), (0.0, 255.0, 102.0));
        assert_eq!(
            hwb2rgb((261.8918918918919, 0.23137254901960785, 0.4784313725490196)),
            (86.0, 59.0, 133.0)
        );
    }
}
