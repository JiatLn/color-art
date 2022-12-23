use crate::{helper::round, Color, ColorSpace};

/// Color channel extraction methods.
///
/// # Examples
/// ```
/// use color_art::Color;
/// use std::str::FromStr;
///
/// let color = Color::from_str("rgba(10, 20, 30, 0.8)").unwrap();
/// assert_eq!(color.red(), 10);
/// assert_eq!(color.green(), 20);
/// assert_eq!(color.blue(), 30);
/// assert_eq!(color.alpha(), 0.8);
///
/// let color = Color::from_str("hsl(90, 100%, 50%)").unwrap();
/// assert_eq!(color.hue(), 90.0);
/// assert_eq!(color.saturation(), 1.0);
/// assert_eq!(color.lightness(), 0.5);
/// ```
impl Color {
    /// Extracts the red channel of a color in the RGB color space.
    pub fn red(&self) -> u8 {
        round(self.rgba.0, 0) as u8
    }
    /// Extracts the green channel of a color in the RGB color space.
    pub fn green(&self) -> u8 {
        round(self.rgba.1, 0) as u8
    }
    /// Extracts the blue channel of a color in the RGB color space.
    pub fn blue(&self) -> u8 {
        round(self.rgba.2, 0) as u8
    }
    /// Extracts the alpha channel of a color in the RGBA color space.
    pub fn alpha(&self) -> f64 {
        round(self.rgba.3, 1)
    }
    /// Extracts the hue channel of a color in the HSL color space.
    pub fn hue(&self) -> f64 {
        match self.space(ColorSpace::Hsl) {
            Ok(hsl) => hsl[0],
            Err(_) => 0.0,
        }
    }
    /// Extracts the saturation channel of a color in the HSL color space.
    pub fn saturation(&self) -> f64 {
        match self.space(ColorSpace::Hsl) {
            Ok(hsl) => hsl[1],
            Err(_) => 0.0,
        }
    }
    /// Extracts the lightness channel of a color in the HSL color space.
    pub fn lightness(&self) -> f64 {
        match self.space(ColorSpace::Hsl) {
            Ok(hsl) => hsl[2],
            Err(_) => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_color_channel() {
        let color = Color::from_str("rgba(10, 20, 30, 0.8)").unwrap();
        assert_eq!(color.red(), 10);
        assert_eq!(color.green(), 20);
        assert_eq!(color.blue(), 30);
        assert_eq!(color.alpha(), 0.8);

        let color = Color::from_str("hsl(90, 100%, 50%)").unwrap();
        assert_eq!(color.hue(), 90.0);
        assert_eq!(color.saturation(), 1.0);
        assert_eq!(color.lightness(), 0.5);
    }
}
