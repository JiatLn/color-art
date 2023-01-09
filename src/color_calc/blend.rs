use crate::{Color, ColorSpace};

pub enum BlendMode {
    /// Multiply 正片叠底
    ///
    /// https://www.w3.org/TR/compositing-1/#blendingmultiply
    Multiply,
    /// Darken 变暗
    Darken,
    /// Lighten 变亮
    Lighten,
    /// Screen 滤色
    Screen,
    /// Overlay 叠加
    Overlay,
    /// Color Burn 颜色加深
    Burn,
    /// Color Dodge 颜色减淡
    Dodge,
}

/// Blends two colors using RGB channel-wise blend functions.
///
/// blend mode formulas taken from [blending](https://www.w3.org/TR/compositing-1/#blending)
pub fn blend(backdrop_color: &Color, source_color: &Color, mode: BlendMode) -> Color {
    let backdrop_vec = backdrop_color.space(ColorSpace::RGB).unwrap();
    let source_vec = source_color.space(ColorSpace::RGB).unwrap();
    let zip_vec: _ = backdrop_vec
        .iter()
        .zip(source_vec.iter())
        .map(|(a, b)| (a / 255., b / 255.));
    let v: Vec<_> = match mode {
        BlendMode::Multiply => zip_vec.map(|(a, b)| multiply(a, b)).collect(),
        BlendMode::Darken => zip_vec.map(|(a, b)| min(a, b)).collect(),
        BlendMode::Lighten => zip_vec.map(|(a, b)| max(a, b)).collect(),
        BlendMode::Screen => zip_vec.map(|(a, b)| screen(a, b)).collect(),
        BlendMode::Overlay => zip_vec.map(|(a, b)| overlay(a, b)).collect(),
        BlendMode::Burn => zip_vec.map(|(a, b)| burn(a, b)).collect(),
        BlendMode::Dodge => zip_vec.map(|(a, b)| dodge(a, b)).collect(),
    };
    let r = v[0] * 255.;
    let g = v[1] * 255.;
    let b = v[2] * 255.;
    Color::new(r, g, b, 1.0)
}

fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn normal(_a: f64, b: f64) -> f64 {
    b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// https://www.w3.org/TR/compositing-1/#blendingscreen
fn screen(a: f64, b: f64) -> f64 {
    1. - (1. - a) * (1. - b)
}

fn overlay(a: f64, b: f64) -> f64 {
    if a <= 0.5 {
        multiply(a, 2. * b)
    } else {
        screen(a, 2. * b - 1.)
    }
}

fn burn(a: f64, b: f64) -> f64 {
    1.0 - (1.0 - a) / b
}

fn dodge(a: f64, b: f64) -> f64 {
    if a == 0. {
        0.
    } else if b == 1. {
        1.
    } else {
        (a / (1. - b)).min(1.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_blend() {
        let color1 = Color::from_str("#4CBBFC").unwrap();
        let color2 = Color::from_str("#EEEE22").unwrap();

        let color = blend(&color1, &color2, BlendMode::Multiply);
        assert_eq!(color.hex(), "#47af22");

        let color = blend(&color1, &color2, BlendMode::Darken);
        assert_eq!(color.hex(), "#4cbb22");

        let color = blend(&color1, &color2, BlendMode::Lighten);
        assert_eq!(color.hex(), "#eeeefc");

        let color = blend(&color1, &color2, BlendMode::Screen);
        assert_eq!(color.hex(), "#f3fafc");

        let color = blend(&color1, &color2, BlendMode::Burn);
        assert_eq!(color.hex(), "#3fb6e9");

        let color = blend(&color1, &color2, BlendMode::Overlay);
        assert_eq!(color.hex(), "#8ef6fa");

        let color = blend(&color1, &color2, BlendMode::Dodge);
        assert_eq!(color.hex(), "#ffffff");
    }
}
