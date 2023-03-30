use crate::{conversion, Color, ColorSpace};
use anyhow::{Ok, Result};

impl Color {
    /// Create a color from RGB values.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_rgb(255.0, 51.0, 153.0).unwrap();
    /// assert_eq!(color.hex(), "#ff3399");
    /// ```
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Result<Color> {
        ColorSpace::RGB.valid(&vec![r, g, b])?;
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from RGBA values.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    /// let color = Color::from_rgba(255.0, 51.0, 153.0, 0.5).unwrap();
    /// assert_eq!(color.rgba(), "rgba(255, 51, 153, 0.5)");
    /// ```
    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Result<Color> {
        ColorSpace::RGBA.valid(&vec![r, g, b, a])?;
        Ok(Color::new(r, g, b, a))
    }
    /// Create a color from HSL values.
    ///
    /// # Example
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::from_hsl(330.0, 1.0, 0.6).unwrap();
    /// assert_eq!(color.hex(), "#ff3399");
    /// ```
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Result<Color> {
        ColorSpace::HSL.valid(&vec![h, s, l])?;
        let (r, g, b) = conversion::hsl::hsl2rgb((h, s, l));
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from HSV values.
    ///
    /// # Example
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_hsv(38.82, 1.0, 1.0).unwrap();
    /// assert_eq!(color.hex(), "#ffa500");
    /// ```
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Result<Color> {
        ColorSpace::HSV.valid(&vec![h, s, v])?;
        let (r, g, b) = conversion::hsv::hsv2rgb((h, s, v));
        Ok(Color::new(r, g, b, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_rgb() {
        let color = Color::from_rgb(255.0, 51.0, 153.0).unwrap();
        assert_eq!(color.hex(), "#ff3399");
    }

    #[test]
    fn test_color_from_rgba() {
        let color = Color::from_rgba(255.0, 51.0, 153.0, 0.5).unwrap();
        assert_eq!(color.rgba(), "rgba(255, 51, 153, 0.5)");
    }

    #[test]
    fn test_color_from_hsl() {
        let color = Color::from_hsl(330.0, 1.0, 0.6).unwrap();
        assert_eq!(color.hex(), "#ff3399");
    }

    #[test]
    fn test_color_from_hsv() {
        let color = Color::from_hsv(38.82, 1.0, 1.0).unwrap();
        assert_eq!(color.hex(), "#ffa500");
    }
}
