use crate::{conversion, parser};
use anyhow::Result;
use std::str::FromStr;

/// Color is a struct that represents a color.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub(crate) rgba: (f64, f64, f64, f64),
}

impl Color {
    /// Creates a new [`Color`].
    pub fn new(r: f64, g: f64, b: f64, alpha: f64) -> Self {
        Color {
            rgba: (r, g, b, alpha),
        }
    }
}

impl FromStr for Color {
    type Err = anyhow::Error;
    /// Creates a new [`Color`] from a string.
    ///
    /// # Examples
    /// ```rust
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let s = "rgb(255, 255, 255)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255.0, 255.0, 255.0, 1.0));
    ///
    /// let s = "rgba(255, 255, 255, 0.5)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255.0, 255.0, 255.0, 0.5));
    ///
    /// let s = "#ffffff";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255.0, 255.0, 255.0, 1.0));
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        let color_str = s.trim().to_lowercase();
        let (r, g, b, a) = match &color_str {
            s if s.starts_with("rgb(") => {
                let (r, g, b) = parser::rgb::parse_rgb_str(s)?;
                (r, g, b, 1.0)
            }
            s if s.starts_with("rgba(") => {
                let (r, g, b, a) = parser::rgba::parse_rgba_str(s)?;
                (r, g, b, a)
            }
            s if s.starts_with('#') => {
                let hex_str = parser::hex::parse_hex_str(s)?;
                let (r, g, b) = conversion::hex::hex2rgb(&hex_str);
                (r, g, b, 1.0)
            }
            _ => return Err(anyhow::anyhow!("{} is not a valid color", s)),
        };
        Ok(Color::new(r, g, b, a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_rgb_str() {
        let s = "rgb(255, 255, 255)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 255.0, 255.0, 1.0));

        let s = "rgb(255, 0, 0)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 0.0, 0.0, 1.0));

        let s = "rgb(0, 255, 0)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(0.0, 255.0, 0.0, 1.0));

        let s = "rgb(255, 255, 0)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 255.0, 0.0, 1.0));

        let s = "rgb(0, 255, 255)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(0.0, 255.0, 255.0, 1.0));
    }

    #[test]
    fn test_color_from_rgb_str_err() {
        let s = "rgb(256, 255, 255)";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(e.to_string(), "r must be between 0 and 255, got 256"),
            _ => panic!("Should have failed"),
        }

        let s = "rgbbb(255, 255, 255)";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(e.to_string(), "rgbbb(255, 255, 255) is not a valid color"),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn test_color_from_rgba_str_err() {
        let s = "rgba(256, 255, 255, 0.5)";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(e.to_string(), "r must be between 0 and 255, got 256"),
            _ => panic!("Should have failed"),
        }

        let s = "rgba(255, 255, 255, 1.5)";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(e.to_string(), "alpha must be between 0 and 1, got 1.5"),
            _ => panic!("Should have failed"),
        }

        let s = "rgba(255, 255, 255, -0.5)";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(e.to_string(), "alpha must be between 0 and 1, got -0.5"),
            _ => panic!("Should have failed"),
        }

        let s = "rgbbb(255, 255, 255, 0.5)";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(
                e.to_string(),
                "rgbbb(255, 255, 255, 0.5) is not a valid color"
            ),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn test_color_from_rgba_str() {
        let s = "rgba(255, 255, 255, 0.5)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 255.0, 255.0, 0.5));

        let s = "rgba(255, 0, 0, 0.5)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 0.0, 0.0, 0.5));

        let s = "rgba(0, 255, 0, 0.5)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(0.0, 255.0, 0.0, 0.5));

        let s = "rgba(255, 255, 0, 0.5)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 255.0, 0.0, 0.5));

        let s = "rgba(0, 255, 255, 0.5)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(0.0, 255.0, 255.0, 0.5));
    }

    #[test]
    fn test_color_from_hex_str() {
        let s = "#ffffff";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 255.0, 255.0, 1.0));

        let s = "#000000";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(0.0, 0.0, 0.0, 1.0));

        let s = "#ff0000";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 0.0, 0.0, 1.0));

        let s = "#00ff00";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(0.0, 255.0, 0.0, 1.0));

        let s = "#ffff00";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(255.0, 255.0, 0.0, 1.0));

        let s = "#00ffff";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color, Color::new(0.0, 255.0, 255.0, 1.0));
    }

    #[test]
    fn test_color_from_hex_str_err() {
        let s = "#gggggg";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(e.to_string(), "Invalid hex string: #gggggg"),
            _ => panic!("Should have failed"),
        }

        let s = "fff";
        let color = Color::from_str(s);
        match color {
            Err(e) => assert_eq!(e.to_string(), "fff is not a valid color"),
            _ => panic!("Should have failed"),
        }
    }
}
