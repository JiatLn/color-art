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
    ///
    /// let s = "hsl(0, 0%, 100%)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255.0, 255.0, 255.0, 1.0));
    ///
    /// let s = "hsv(0, 0%, 100%)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255.0, 255.0, 255.0, 1.0));
    ///
    /// let s = "deeppink";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255.0, 20.0, 147.0, 1.0));
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
            s if s.starts_with("hsl(") => {
                let hsl = parser::hsl::parse_hsl_str(s)?;
                let (r, g, b) = conversion::hsl::hsl2rgb(hsl);
                (r, g, b, 1.0)
            }
            s if s.starts_with("hsv(") => {
                let hsv = parser::hsv::parse_hsv_str(s)?;
                let (r, g, b) = conversion::hsv::hsv2rgb(hsv);
                (r, g, b, 1.0)
            }
            s if s.starts_with("hwb(") => {
                let hwb = parser::hwb::parse_hwb_str(s)?;
                let (r, g, b) = conversion::hwb::hwb2rgb(hwb);
                (r, g, b, 1.0)
            }
            s if s.starts_with("cmyk(") => {
                let cmyk = parser::cmyk::parse_cmyk_str(s)?;
                let (r, g, b) = conversion::cmyk::cmyk2rgb(cmyk);
                (r, g, b, 1.0)
            }
            s if s.starts_with("xyz(") => {
                let xyz = parser::xyz::parse_xyz_str(s)?;
                let (r, g, b) = conversion::xyz::xyz2rgb(xyz);
                (r, g, b, 1.0)
            }
            _ => {
                let found = crate::W3CX11.get(s);
                match found {
                    Some(hex) => {
                        let (r, g, b) = conversion::hex::hex2rgb(hex);
                        (r, g, b, 1.0)
                    }
                    None => anyhow::bail!("{} is not a valid color", s),
                }
            }
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
        assert!(color.is_err());

        let s = "rgbbb(255, 255, 255)";
        let color = Color::from_str(s);
        assert!(color.is_err());
    }

    #[test]
    fn test_color_from_rgba_str_err() {
        let s = "rgba(256, 255, 255, 0.5)";
        let color = Color::from_str(s);
        assert!(color.is_err());

        let s = "rgba(255, 255, 255, 1.5)";
        let color = Color::from_str(s);
        assert!(color.is_err());

        let s = "rgba(255, 255, 255, -0.5)";
        let color = Color::from_str(s);
        assert!(color.is_err());

        let s = "rgbbb(255, 255, 255, 0.5)";
        let color = Color::from_str(s);
        assert!(color.is_err());
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

    #[test]
    fn test_color_from_hsl_str() {
        let s = "hsl(180, 100%, 50%)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color.hsl(), "hsl(180, 100%, 50%)");
        assert_eq!(color.rgb(), "rgb(0, 255, 255)");
    }

    #[test]
    fn test_color_from_hsv_str() {
        let s = "hsv(180, 100%, 100%)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color.hsv(), "hsv(180, 100%, 100%)");
        assert_eq!(color.rgb(), "rgb(0, 255, 255)");
    }

    #[test]
    fn test_color_from_hwb_str() {
        let s = "hwb(180, 0%, 0%)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color.hwb(), "hwb(180, 0%, 0%)");
        assert_eq!(color.rgb(), "rgb(0, 255, 255)");
    }

    #[test]
    fn test_color_from_name_str() {
        let s = "red";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color.rgb(), "rgb(255, 0, 0)");
    }

    #[test]
    fn test_color_from_cmyk_str() {
        let color = Color::from_str("cmyk(0, 100%, 100%, 0)").unwrap();
        assert_eq!(color.hex(), "#ff0000");

        let color = Color::from_str("cmyk(100%, 0, 100%, 0)").unwrap();
        assert_eq!(color.hex(), "#00ff00");

        let color = Color::from_str("cmyk(100%, 100%, 0, 0)").unwrap();
        assert_eq!(color.hex(), "#0000ff");

        let color = Color::from_str("cmyk(0, 0, 0, 100%)").unwrap();
        assert_eq!(color.hex(), "#000000");

        let color = Color::from_str("cmyk(0, 0, 0, 0)").unwrap();
        assert_eq!(color.hex(), "#ffffff");

        let color = Color::from_str("cmyk(20%, 80%, 0, 0)").unwrap();
        assert_eq!(color.hex(), "#cc33ff");

        let color = Color::from_str("cmyk(35%, 0, 60%, 0)").unwrap();
        assert_eq!(color.hex(), "#a6ff66");
    }

    #[test]
    fn test_color_from_xyz_str() {
        let color = Color::from_str("xyz(0.412453, 0.212671, 0.019334)").unwrap();
        assert_eq!(color.rgb(), "rgb(255, 0, 0)");

        let color = Color::from_str("xyz(0.70047, 0.723315, 1.048516)").unwrap();
        assert_eq!(color.rgb(), "rgb(162, 184, 255)");
    }
}
