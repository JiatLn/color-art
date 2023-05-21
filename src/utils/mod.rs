pub mod blend_fn;
pub mod hex;
pub mod math;

pub use math::*;

pub(crate) fn round(val: f64, precision: u32) -> f64 {
    let factor = (10.0_f64).powi(precision as i32);
    let val = (val * factor).round() / factor;
    if val == -0.0 {
        0.0
    } else {
        val
    }
}

/// normalize color values 0..255 to 0..1
pub(crate) fn normalize_color(color: &[f64]) -> Vec<f64> {
    color
        .iter()
        .map(|&c| c / 255.0)
        .collect()
}
