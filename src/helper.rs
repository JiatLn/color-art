pub(crate) fn round(val: f64, precision: u32) -> f64 {
    let factor = 10.0_f64.powi(precision as i32);
    (val * factor).round() / factor
}
