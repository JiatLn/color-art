use crate::{Color, ColorSpace};

/// ### blend mode enum
///
/// The blend mode defines the formula that must be used to mix the colors with the backdrop.
pub enum BlendMode {
    /// ### multiply blend mode
    ///
    /// The source color is multiplied by the destination color and replaces the destination.
    ///
    /// The resultant color is always at least as dark as either the source or destination color. Multiplying any color with black results in black. Multiplying any color with white preserves the original color.
    Multiply,
    /// ### darken blend mode
    ///
    /// Selects the darker of the backdrop and source colors.
    ///
    /// The backdrop is replaced with the source where the source is darker; otherwise, it is left unchanged.
    Darken,
    /// ### lighten blend mode
    ///
    /// Selects the lighter of the backdrop and source colors.
    ///
    /// The backdrop is replaced with the source where the source is lighter; otherwise, it is left unchanged.
    Lighten,
    /// ### screen blend mode
    ///
    /// Multiplies the complements of the backdrop and source color values, then complements the result.
    ///
    /// The result color is always at least as light as either of the two constituent colors. Screening any color with white produces white; screening with black leaves the original color unchanged. The effect is similar to projecting multiple photographic slides simultaneously onto a single screen.
    Screen,
    /// ### overlay blend mode
    ///
    /// Multiplies or screens the colors, depending on the backdrop color value.
    ///
    /// Source colors overlay the backdrop while preserving its highlights and shadows. The backdrop color is not replaced but is mixed with the source color to reflect the lightness or darkness of the backdrop.
    Overlay,
    /// ### color-burn blend mode
    ///
    /// Darkens the backdrop color to reflect the source color. Painting with white produces no change.
    ColorBurn,
    /// ### color-dodge blend mode
    ///
    /// Brightens the backdrop color to reflect the source color. Painting with black produces no changes.
    ColorDodge,
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
        BlendMode::ColorBurn => zip_vec.map(|(a, b)| burn(a, b)).collect(),
        BlendMode::ColorDodge => zip_vec.map(|(a, b)| dodge(a, b)).collect(),
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

        let color = blend(&color1, &color2, BlendMode::ColorBurn);
        assert_eq!(color.hex(), "#3fb6e9");

        let color = blend(&color1, &color2, BlendMode::Overlay);
        assert_eq!(color.hex(), "#8ef6fa");

        let color = blend(&color1, &color2, BlendMode::ColorDodge);
        assert_eq!(color.hex(), "#ffffff");
    }
}
