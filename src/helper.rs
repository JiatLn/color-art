pub(crate) fn round(val: f64, precision: u32) -> f64 {
    let factor = 10.0_f64.powi(precision as i32);
    (val * factor).round() / factor
}

pub(crate) fn normalize_color(color: (f64, f64, f64)) -> (f64, f64, f64) {
    let (r, g, b) = color;
    (r / 255.0, g / 255.0, b / 255.0)
}
