use crate::utils::*;

/// [HSV to RGB color conversion](https://www.rapidtables.com/convert/color/hsv-to-rgb.html)
pub fn hsv2rgb(color: &[f64]) -> Vec<f64> {
    let h = color[0];
    let s = color[1];
    let v = color[2];

    let c = v * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;

    let rgb = match h {
        h if h >= 0.0 && h < 60.0 => vec![c, x, 0.0],
        h if h >= 60.0 && h < 120.0 => vec![x, c, 0.0],
        h if h >= 120.0 && h < 180.0 => vec![0.0, c, x],
        h if h >= 180.0 && h < 240.0 => vec![0.0, x, c],
        h if h >= 240.0 && h < 300.0 => vec![x, 0.0, c],
        h if h >= 300.0 && h < 360.0 => vec![c, 0.0, x],
        _ => panic!("Hue must be between 0 and 360"),
    };

    rgb.iter().map(|&x| (x + m) * 255.0).collect()
}

/// [RGB to HSV color conversion](https://www.rapidtables.com/convert/color/rgb-to-hsv.html)
pub fn rgb2hsv(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let r = color[0];
    let g = color[1];
    let b = color[2];

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let mut h = 0.0;
    let mut s = 0.0;
    let v = max;

    if delta != 0.0 {
        h = match max {
            x if x == r => 60.0 * (((g - b) / delta) % 6.0),
            x if x == g => 60.0 * ((b - r) / delta + 2.0),
            x if x == b => 60.0 * ((r - g) / delta + 4.0),
            _ => panic!(),
        };

        if h < 0.0 {
            h = h + 360.0;
        }
    }

    if max != 0.0 {
        s = delta / max;
    }

    vec![h, s, v]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv2rgb() {
        let hsv = vec![330.0, 0.8, 1.0];
        let rgb = hsv2rgb(&hsv);
        assert_eq!(rgb, vec![255.0, 50.999999999999986, 153.0]);

        let hsv = vec![0.0, 0.0, 0.0];
        let rgb = hsv2rgb(&hsv);
        assert_eq!(rgb, vec![0.0, 0.0, 0.0]);

        let hsv = vec![0.0, 0.0, 1.0];
        let rgb = hsv2rgb(&hsv);
        assert_eq!(rgb, vec![255.0, 255.0, 255.0]);
    }

    #[test]
    fn test_rgb2hsv() {
        let rgb = vec![255.0, 51.0, 153.0];
        let hsv = rgb2hsv(&rgb);
        assert_eq!(hsv, vec![330.0, 0.8, 1.0]);

        let rgb = vec![0.0, 0.0, 0.0];
        let hsv = rgb2hsv(&rgb);
        assert_eq!(hsv, vec![0.0, 0.0, 0.0]);

        let rgb = vec![255.0, 255.0, 255.0];
        let hsv = rgb2hsv(&rgb);
        assert_eq!(hsv, vec![0.0, 0.0, 1.0]);
    }
}
