use crate::utils::*;

/// [RGB to CMYK color conversion](https://www.rapidtables.com/convert/color/rgb-to-cmyk.html)
pub fn rgb2cmyk(color: &[f64]) -> Vec<f64> {
    let color = normalize_color(color);
    let r = color[0];
    let g = color[1];
    let b = color[2];

    let rgb_max = f64::max(f64::max(r, g), b);

    let k = 1.0 - rgb_max;
    let c = (1.0 - r - k) * (1.0 - k);
    let m = (1.0 - g - k) * (1.0 - k);
    let y = (1.0 - b - k) * (1.0 - k);

    vec![c, m, y, k]
}

/// [CMYK to RGB color conversion](https://www.rapidtables.com/convert/color/cmyk-to-rgb.html)
pub fn cmyk2rgb(color: &[f64]) -> Vec<f64> {
    let c = color[0];
    let m = color[1];
    let y = color[2];
    let k = color[3];

    let r = (1.0 - c) * (1.0 - k) * 255.0;
    let g = (1.0 - m) * (1.0 - k) * 255.0;
    let b = (1.0 - y) * (1.0 - k) * 255.0;

    vec![r, g, b]
}
