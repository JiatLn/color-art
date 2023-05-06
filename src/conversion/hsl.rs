use crate::helper::*;

/// [HSL to RGB color conversion](https://www.rapidtables.com/convert/color/hsl-to-rgb.html)
pub fn hsl2rgb(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (h, s, l) = color;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

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

/// [RGB to HSL color conversion](https://www.rapidtables.com/convert/color/rgb-to-hsl.html)
pub fn rgb2hsl(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    let mut h = 0.0;
    let mut s = 0.0;
    let l = (max + min) / 2.0;

    let delta = max - min;

    if delta != 0.0 {
        h = match max {
            x if x == r => 60.0 * (((g - b) / delta) % 6.0),
            x if x == g => 60.0 * (((b - r) / delta) + 2.0),
            x if x == b => 60.0 * (((r - g) / delta) + 4.0),
            _ => panic!(),
        };

        if h < 0. {
            h = h + 360.
        }

        s = delta / (1.0 - (2.0 * l - 1.0).abs());
    }

    (round(h, 4), round(s, 4), round(l, 4))
}
