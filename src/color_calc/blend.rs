use crate::{utils::blend_fn::*, Color, ColorSpace};

/// ### blend mode enum
///
/// The blend mode defines the formula that must be used to mix the colors with the backdrop.
pub enum BlendMode {
    /// ### normal blend mode
    ///
    /// This is the default attribute which specifies no blending. The blending formula simply selects the source color.
    Normal,
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
    /// ### hard-light blend mode
    ///
    /// Multiplies or screens the colors, depending on the source color value. The effect is similar to shining a harsh spotlight on the backdrop.
    HardLight,
    /// ### soft-light blend mode
    ///
    /// Darkens or lightens the colors, depending on the source color value. The effect is similar to shining a diffused spotlight on the backdrop.
    SoftLight,
    /// ### difference blend mode
    ///
    /// Subtracts the darker of the two constituent colors from the lighter color.
    ///
    /// Painting with white inverts the backdrop color; painting with black produces no change.
    Difference,
    /// ### exclusion blend mode
    ///
    /// Produces an effect similar to that of the Difference mode but lower in contrast. Painting with white inverts the backdrop color; painting with black produces no change.
    Exclusion,
}

/// Blends two colors using RGB channel-wise blend functions.
///
/// # Parameters
///
/// * `backdrop_color` - The backdrop color.
/// * `source_color` - The source color.
/// * `mode` - The blend mode to use. See [BlendMode](enum.BlendMode.html) for more information.
///
/// Supports the following blend modes:
///
/// * Normal
/// * Multiply
/// * Darken
/// * Lighten
/// * Screen
/// * Overlay
/// * ColorBurn
/// * ColorDodge
/// * HardLight
/// * SoftLight
/// * Difference
/// * Exclusion
///
/// The blend mode formulas taken from [blending](https://www.w3.org/TR/compositing-1/#blending).
///
/// # Examples
///
/// ```
/// use color_art::{color, BlendMode, blend};
///
/// let color1 = color!(#4cbbfc);
/// let color2 = color!(#eeee22);
///
/// let blended_color = blend(&color1, &color2, BlendMode::Overlay);
/// assert_eq!(blended_color.hex(), "#8ef6fa");
/// ```
pub fn blend(backdrop_color: &Color, source_color: &Color, mode: BlendMode) -> Color {
    let backdrop_vec = backdrop_color.vec_of(ColorSpace::RGB);
    let source_vec = source_color.vec_of(ColorSpace::RGB);

    let zip_vec: _ = backdrop_vec
        .iter()
        .zip(source_vec.iter())
        .map(|(a, b)| (a / 255.0, b / 255.0));

    let v: Vec<_> = match mode {
        BlendMode::Normal => zip_vec.map(|(a, b)| normal(a, b)).collect(),
        BlendMode::Multiply => zip_vec.map(|(a, b)| multiply(a, b)).collect(),
        BlendMode::Darken => zip_vec.map(|(a, b)| min(a, b)).collect(),
        BlendMode::Lighten => zip_vec.map(|(a, b)| max(a, b)).collect(),
        BlendMode::Screen => zip_vec.map(|(a, b)| screen(a, b)).collect(),
        BlendMode::Overlay => zip_vec.map(|(a, b)| overlay(a, b)).collect(),
        BlendMode::ColorBurn => zip_vec.map(|(a, b)| burn(a, b)).collect(),
        BlendMode::ColorDodge => zip_vec.map(|(a, b)| dodge(a, b)).collect(),
        BlendMode::HardLight => zip_vec.map(|(a, b)| hard_light(a, b)).collect(),
        BlendMode::SoftLight => zip_vec.map(|(a, b)| soft_light(a, b)).collect(),
        BlendMode::Difference => zip_vec.map(|(a, b)| difference(a, b)).collect(),
        BlendMode::Exclusion => zip_vec.map(|(a, b)| exclusion(a, b)).collect(),
    };

    let r = v[0] * 255.0;
    let g = v[1] * 255.0;
    let b = v[2] * 255.0;

    Color::new(r, g, b, 1.0)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use BlendMode::*;

    #[test]
    fn test_blend() {
        let c1 = color!(#4cbbfc);
        let c2 = color!(#eeee22);

        let color = blend(&c1, &c2, Multiply);
        assert_eq!(color.hex(), "#47af22");

        let color = blend(&c1, &c2, Darken);
        assert_eq!(color.hex(), "#4cbb22");

        let color = blend(&c1, &c2, Lighten);
        assert_eq!(color.hex(), "#eeeefc");

        let color = blend(&c1, &c2, Screen);
        assert_eq!(color.hex(), "#f3fafc");

        let color = blend(&c1, &c2, ColorBurn);
        assert_eq!(color.hex(), "#3fb6e9");

        let color = blend(&c1, &c2, Overlay);
        assert_eq!(color.hex(), "#8ef6fa");

        let color = blend(&c1, &c2, ColorDodge);
        assert_eq!(color.hex(), "#fff");

        let color = blend(&c1, &c2, HardLight);
        assert_eq!(color.hex(), "#e7f643");
    }
}
