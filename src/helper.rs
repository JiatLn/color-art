pub(crate) fn round(val: f64, precision: u32) -> f64 {
    let factor = 10.0_f64.powi(precision as i32);
    let val = (val * factor).round() / factor;
    if val == -0.0 {
        0.0
    } else {
        val
    }
}

/// normalize color values 0..255 to 0..1
pub(crate) fn normalize_color(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = color;
    (r / 255.0, g / 255.0, b / 255.0)
}

/// Convert a vector of f64 to a tuple of f64
pub(crate) fn vec2tuple(vec: Vec<f64>) -> (f64, f64, f64) {
    (vec[0], vec[1], vec[2])
}
