use crate::helper::*;

/// [RGB to CMYK color conversion](https://www.rapidtables.com/convert/color/rgb-to-cmyk.html)
pub fn rgb2cmyk(color: (f64, f64, f64)) -> (f64, f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let rgb_max = f64::max(f64::max(r, g), b);

    let k = 1. - rgb_max;
    let c = (1. - r - k) * (1. - k);
    let m = (1. - g - k) * (1. - k);
    let y = (1. - b - k) * (1. - k);

    (c, y, m, k)
}

/// [CMYK to RGB color conversion](https://www.rapidtables.com/convert/color/cmyk-to-rgb.html)
pub fn cmyk2rgb(color: (f64, f64, f64, f64)) -> (f64, f64, f64) {
    let (c, m, y, k) = color;

    let r = (1. - c) * (1. - k) * 255.;
    let g = (1. - m) * (1. - k) * 255.;
    let b = (1. - y) * (1. - k) * 255.;

    (r, g, b)
}
