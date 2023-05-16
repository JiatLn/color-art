use crate::utils::*;

/// [RGB to CMYK color conversion](https://www.rapidtables.com/convert/color/rgb-to-cmyk.html)
pub fn rgb2cmyk(color: (f64, f64, f64)) -> (f64, f64, f64, f64) {
    let (r, g, b) = normalize_color(color);

    let rgb_max = f64::max(f64::max(r, g), b);

    let k = 1.0 - rgb_max;
    let c = (1.0 - r - k) * (1.0 - k);
    let m = (1.0 - g - k) * (1.0 - k);
    let y = (1.0 - b - k) * (1.0 - k);

    (c, m, y, k)
}

/// [CMYK to RGB color conversion](https://www.rapidtables.com/convert/color/cmyk-to-rgb.html)
pub fn cmyk2rgb(color: (f64, f64, f64, f64)) -> (f64, f64, f64) {
    let (c, m, y, k) = color;

    let r = (1.0 - c) * (1.0 - k) * 255.0;
    let g = (1.0 - m) * (1.0 - k) * 255.0;
    let b = (1.0 - y) * (1.0 - k) * 255.0;

    (r, g, b)
}
