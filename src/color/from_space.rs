use crate::{ conversion, data::hex_of_name, Color, ColorSpace };
use anyhow::{ Ok, Result };

impl Color {
    /// Create a color from RGB values.
    ///
    /// # Parameters
    ///
    /// - `r`: Red value (0-255)
    /// - `g`: Green value (0-255)
    /// - `b`: Blue value (0-255)
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_rgb(255, 51, 153).unwrap();
    /// assert_eq!(color.hex(), "#f39");
    /// ```
    pub fn from_rgb<T>(r: T, g: T, b: T) -> Result<Self> where T: Into<f64> {
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
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    /// let color = Color::from_rgba(255, 51, 153, 0.5).unwrap();
    /// assert_eq!(color.rgba(), "rgba(255, 51, 153, 0.5)");
    /// ```
    pub fn from_rgba<T>(r: T, g: T, b: T, a: f64) -> Result<Self> where T: Into<f64> {
        let r = r.into();
        let g = g.into();
        let b = b.into();
        ColorSpace::RGBA.valid(&vec![r, g, b, a])?;
        Ok(Color::new(r, g, b, a))
    }
    /// Create a color from HSL values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    ///
    /// let color = Color::from_hsl(330.0, 1.0, 0.6).unwrap();
    /// assert_eq!(color.hex(), "#f39");
    /// ```
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Result<Self> {
        let hsl = vec![h, s, l];
        ColorSpace::HSL.valid(&hsl)?;
        let rgb = conversion::hsl::hsl2rgb(&hsl);
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from HSV values.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_hsv(38.82, 1.0, 1.0).unwrap();
    /// assert_eq!(color.hex(), "#ffa500");
    /// ```
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Result<Self> {
        let hsv = vec![h, s, v];
        ColorSpace::HSV.valid(&hsv)?;
        let rgb = conversion::hsv::hsv2rgb(&hsv);
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from CMYK values.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_cmyk(0.0, 0.8, 0.4, 0.0).unwrap();
    /// assert_eq!(color.hex(), "#f39");
    /// ```
    pub fn from_cmyk(c: f64, m: f64, y: f64, k: f64) -> Result<Self> {
        let cmyk = vec![c, m, y, k];
        ColorSpace::CMYK.valid(&cmyk)?;
        let rgb = conversion::cmyk::cmyk2rgb(&cmyk);
        let r = rgb[0];
        let g = rgb[1];
        let b = rgb[2];
        Ok(Color::new(r, g, b, 1.0))
    }
    /// Create a color from a hex string.
    ///
    /// # Examples
    ///
    /// ```
    /// use color_art::Color;
    ///
    /// let color = Color::from_hex("#ff3399").unwrap();
    /// assert_eq!(color.hex(), "#f39");
    ///
    /// let color = Color::from_hex("#ff339933").unwrap();
    /// assert_eq!(color.hex(), "#f393");
    /// ```
    pub fn from_hex(hex_str: &str) -> Result<Self> {
        ColorSpace::valid_hex(hex_str)?;
        let color_vec = match hex_str.len() {
            4 | 7 => conversion::hex::hex2rgb(hex_str),
            5 | 9 => conversion::hex::hex2rgba(hex_str),
            _ => anyhow::bail!("Got a error hex string!"),
        };
        let r = color_vec[0];
        let g = color_vec[1];
        let b = color_vec[2];
        let a = if color_vec.len() == 4 { color_vec[3] } else { 1.0 };
        Ok(Color::new(r, g, b, a))
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
    /// assert_eq!(color.hex(), "#ff0");
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
        assert_eq!(color.hex(), "#f39");

        let color = Color::from_rgb(255.0, 51.0, 153.0).unwrap();
        assert_eq!(color.hex(), "#f39");
    }

    #[test]
    fn test_color_from_rgba() {
        let color = Color::from_rgba(255.0, 51.0, 153.0, 0.5).unwrap();
        assert_eq!(color.rgba(), "rgba(255, 51, 153, 0.5)");
    }

    #[test]
    fn test_color_from_hsl() {
        let color = Color::from_hsl(330.0, 1.0, 0.6).unwrap();
        assert_eq!(color.hex(), "#f39");
    }

    #[test]
    fn test_color_from_hsv() {
        let color = Color::from_hsv(38.82, 1.0, 1.0).unwrap();
        assert_eq!(color.hex(), "#ffa500");
    }

    #[test]
    fn test_color_from_cmyk() {
        let color = Color::from_cmyk(0.0, 0.8, 0.4, 0.0).unwrap();
        assert_eq!(color.hex(), "#f39");

        let color = Color::from_cmyk(0.2, 0.8, 0.0, 0.0).unwrap();
        assert_eq!(color.hex(), "#c3f");

        let color = Color::from_cmyk(2.0, 0.0, 0.0, 1.0);
        assert!(color.is_err());
    }
}
