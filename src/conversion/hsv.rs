use crate::helper::*;

/// [HSV to RGB color conversion](https://www.rapidtables.com/convert/color/hsv-to-rgb.html)
pub fn hsv2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (h, s, v) = color;

    let c = v * s;
    let x = c * (1.0 - (((h / 60.0) % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h >= 0.0 && h < 60.0 => (c, x, 0.0),
        h if h >= 60.0 && h < 120.0 => (x, c, 0.0),
        h if h >= 120.0 && h < 180.0 => (0.0, c, x),
        h if h >= 180.0 && h < 240.0 => (0.0, x, c),
        h if h >= 240.0 && h < 300.0 => (x, 0.0, c),
        h if h >= 300.0 && h < 360.0 => (c, 0.0, x),
        _ => panic!(),
    };

    ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0)
}

/// [RGB to HSV color conversion](https://www.rapidtables.com/convert/color/rgb-to-hsv.html)
pub fn rgb2hsv(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

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

    (round(h, 4), round(s, 4), round(v, 4))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv2rgb() {
        let hsv = (0.0, 0.0, 0.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (0.0, 0.0, 0.0));

        let hsv = (0.0, 0.0, 1.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (255.0, 255.0, 255.0));

        let hsv = (0.0, 1.0, 1.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (255.0, 0.0, 0.0));

        let hsv = (120.0, 1.0, 1.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (0.0, 255.0, 0.0));

        let hsv = (240.0, 1.0, 1.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (0.0, 0.0, 255.0));

        let hsv = (60.0, 1.0, 1.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (255.0, 255.0, 0.0));

        let hsv = (180.0, 1.0, 1.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (0.0, 255.0, 255.0));

        let hsv = (300.0, 1.0, 1.0);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (255.0, 0.0, 255.0));

        let hsv = (0.0, 1.0, 0.5);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (127.5, 0.0, 0.0));

        let hsv = (120.0, 1.0, 0.5);
        let rgb = hsv2rgb(hsv);
        assert_eq!(rgb, (0.0, 127.5, 0.0));
    }

    #[test]
    fn test_rgb2hsv() {
        let rgb = (0.0, 0.0, 0.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (0.0, 0.0, 0.0));

        let rgb = (255.0, 255.0, 255.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (0.0, 0.0, 1.0));

        let rgb = (255.0, 0.0, 0.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (0.0, 1.0, 1.0));

        let rgb = (0.0, 255.0, 0.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (120.0, 1.0, 1.0));

        let rgb = (0.0, 0.0, 255.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (240.0, 1.0, 1.0));

        let rgb = (255.0, 255.0, 0.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (60.0, 1.0, 1.0));

        let rgb = (0.0, 255.0, 255.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (180.0, 1.0, 1.0));

        let rgb = (255.0, 0.0, 255.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (300.0, 1.0, 1.0));

        let rgb = (127.5, 0.0, 0.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (0.0, 1.0, 0.5));

        let rgb = (0.0, 127.5, 0.0);
        let hsv = rgb2hsv(rgb);
        assert_eq!(hsv, (120.0, 1.0, 0.5));
    }
}
