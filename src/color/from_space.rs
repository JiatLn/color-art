use crate::{conversion, data::hex_of_name, Color, ColorSpace};
use anyhow::{Ok, Result};

impl Color {
    /// Create a color from RGB values.
    ///
    /// # Parameters
    ///
    /// - `r`: Red value (0-255)
    /// - `g`: Green value (0-255)
    /// - `b`: Blue value (0-255)
    ///
    /// # Example
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_rgb(255, 51, 153).unwrap();
    /// assert_eq!(color.hex(), "#ff3399");
    /// ```
    pub fn from_rgb<T>(r: T, g: T, b: T) -> Result<Self>
    where
        T: Into<f64>,
    {
        let r = r.into();
        let g = g.into();
        let b = b.into();
        ColorSpace::RGB.valid(&vec![r, g, b])?;
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from RGBA values.
    ///
    /// # Parameters
    ///
    /// - `r`: Red value (0-255)
    /// - `g`: Green value (0-255)
    /// - `b`: Blue value (0-255)
    /// - `a`: Alpha value (0-1)
    ///
    /// # Example
    ///
    /// ```
    /// use color_art::Color;
    /// let color = Color::from_rgba(255, 51, 153, 0.5).unwrap();
    /// assert_eq!(color.rgba(), "rgba(255, 51, 153, 0.5)");
    /// ```
    pub fn from_rgba<T>(r: T, g: T, b: T, a: f64) -> Result<Self>
    where
        T: Into<f64>,
    {
        let r = r.into();
        let g = g.into();
        let b = b.into();
        ColorSpace::RGBA.valid(&vec![r, g, b, a])?;
        Ok(Color::new(r, g, b, a))
    }
    /// Create a color from HSL values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::from_hsl(330.0, 1.0, 0.6).unwrap();
    /// assert_eq!(color.hex(), "#ff3399");
    /// ```
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Result<Self> {
        ColorSpace::HSL.valid(&vec![h, s, l])?;
        let (r, g, b) = conversion::hsl::hsl2rgb((h, s, l));
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from HSV values.
    ///
    /// # Example
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_hsv(38.82, 1.0, 1.0).unwrap();
    /// assert_eq!(color.hex(), "#ffa500");
    /// ```
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Result<Self> {
        ColorSpace::HSV.valid(&vec![h, s, v])?;
        let (r, g, b) = conversion::hsv::hsv2rgb((h, s, v));
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from CMYK values.
    ///
    /// # Example
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_cmyk(0.0, 0.8, 0.4, 0.0).unwrap();
    /// assert_eq!(color.hex(), "#ff3399");
    /// ```
    pub fn from_cmyk(c: f64, m: f64, y: f64, k: f64) -> Result<Self> {
        ColorSpace::CMYK.valid(&vec![c, m, y, k])?;
        let (r, g, b) = conversion::cmyk::cmyk2rgb((c, m, y, k));
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from a hex string.
    ///
    /// # Example
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_hex("#ff3399").unwrap();
    /// assert_eq!(color.hex(), "#ff3399");
    /// ```
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        ColorSpace::valid_hex(hex_str)?;
        let (r, g, b) = conversion::hex::hex2rgb(hex_str);
        Ok(Color::new(r, g, b, 1.0))
    }

    /// Create a color from a color name.
    ///
    /// Currently supported color names are:
    ///
    /// - English color names from [X11_color_names](https://en.wikipedia.org/wiki/X11_color_names)
    /// - 中国传统色 (Chinese traditional colors)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::from_name("yellow").unwrap();
    /// assert_eq!(color.hex(), "#ffff00");
    ///
    /// let color = Color::from_name("水绿").unwrap();
    /// assert_eq!(color.hex(), "#8cc269");
    /// ```
    pub fn from_name(name: &str) -> Result<Self> {
        let found = hex_of_name(name);
        match found {
            Some(hex) => Color::from_hex(hex),
            None => anyhow::bail!("Invalid color name: {}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_rgb() {
        let color = Color::from_rgb(255, 51, 153).unwrap();
        assert_eq!(color.hex(), "#ff3399");

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

    #[test]
    fn test_color_from_cmyk() {
        let color = Color::from_cmyk(0.0, 0.8, 0.4, 0.0).unwrap();
        assert_eq!(color.hex(), "#ff3399");

        let color = Color::from_cmyk(0.2, 0.8, 0.0, 0.0).unwrap();
        assert_eq!(color.hex(), "#cc33ff");

        let color = Color::from_cmyk(2.0, 0.0, 0.0, 1.0);
        assert!(color.is_err());
    }
}
