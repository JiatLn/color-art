use crate::conversion::hsl;
use crate::utils::*;

/// Convert RGB to HWB.
///
/// Reference from [Converting sRGB Colors to HWB](https://w3c.github.io/csswg-drafts/css-color/#rgb-to-hwb)
pub fn rgb2hwb(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let r = color[0];
    let g = color[1];
    let b = color[2];

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let hsl = hsl::rgb2hsl(&color);

    let hue = hsl[0];
    let whiteness = min;
    let blackness = 1.0 - max;

    vec![hue, whiteness, blackness]
}

/// Reference from [Converting HWB Colors to sRGB](https://w3c.github.io/csswg-drafts/css-color/#hsl-to-rgb)
pub fn hwb2rgb(color: &[f64]) -> Vec<f64> {
    let hue = color[0];
    let whiteness = color[1];
    let blackness = color[2];

    if whiteness + blackness >= 1.0 {
        let gray = whiteness / (whiteness + blackness);
        return vec![gray, gray, gray];
    }
    let mut rgb_vec = hsl::hsl2rgb(&[hue, 1.0, 0.5]);

    for channel in rgb_vec.iter_mut().take(3) {
        *channel *= 1.0 - whiteness - blackness;
        *channel += whiteness * 255.0;
        *channel = round(*channel, 0);
    }
    rgb_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb2hwb() {
        assert_eq!(rgb2hwb(&[0.0, 255.0, 102.0]), vec![144.0, 0.0, 0.0]);
        assert_eq!(
            rgb2hwb(&[86.0, 59.0, 133.0]),
            vec![261.8918918918919, 0.23137254901960785, 0.4784313725490196]
        );
    }

    #[test]
    fn test_hwb2rgb() {
        assert_eq!(hwb2rgb(&[144.0, 0.0, 0.0]), vec![0.0, 255.0, 102.0]);
        assert_eq!(
            hwb2rgb(&[261.8918918918919, 0.23137254901960785, 0.4784313725490196]),
            vec![86.0, 59.0, 133.0]
        );
    }
}
