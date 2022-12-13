use crate::parser;
use anyhow::Result;
use std::str::FromStr;

/// Color is a struct that represents a color.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub(crate) rgba: (f64, f64, f64, f64),
}

impl Color {
    /// Creates a new [`Color`].
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { rgba: (r, g, b, a) }
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
    /// let s = "rgb(0, 0, 0)";
    /// let color = s.parse::<Color>().unwrap();
    /// assert_eq!(color, Color::new(0.0, 0.0, 0.0, 1.0));
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        let color_str = s.trim().to_lowercase();
        match &color_str {
            s if s.starts_with("rgb(") => {
                let (r, g, b) = parser::rgb::parser_rgb_str(s)?;
                Ok(Color::new(r, g, b, 1.0))
            }
            _ => Err(anyhow::anyhow!("{} is not a valid color", s)),
        }
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
}
