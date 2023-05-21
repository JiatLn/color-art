use crate::{ conversion, data::hex_of_name, parser, Color };
use anyhow::Result;
use std::str::FromStr;

impl FromStr for Color {
    type Err = anyhow::Error;
    /// Creates a new [`Color`] from a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use color_art::Color;
    /// use std::str::FromStr;
    ///
    /// let s = "rgb(255, 255, 255)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255, 255, 255, 1.0));
    ///
    /// let s = "rgba(255, 255, 255, 0.5)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255, 255, 255, 0.5));
    ///
    /// let s = "#ffffff";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255, 255, 255, 1.0));
    ///
    /// let s = "hsl(0, 0%, 100%)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255, 255, 255, 1.0));
    ///
    /// let s = "hsv(0, 0%, 100%)";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255, 255, 255, 1.0));
    ///
    /// let s = "deeppink";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(255, 20, 147, 1.0));
    ///
    /// let s = "水绿";
    /// let color = Color::from_str(s).unwrap();
    /// assert_eq!(color, Color::new(140, 194, 105, 1.0));
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        let color_str = s.trim().to_lowercase();
        let (r, g, b, a) = match &color_str {
            s if s.starts_with("rgb(") => {
                let (r, g, b) = parser::rgb::parse_rgb_str(s)?;
                (r, g, b, 1.0)
            }
            s if s.starts_with("rgba(") => parser::rgba::parse_rgba_str(s)?,
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
            s if s.starts_with("hsla(") => {
                let (h, s, l, a) = parser::hsla::parse_hsla_str(s)?;
                let (r, g, b) = conversion::hsl::hsl2rgb((h, s, l));
                (r, g, b, a)
            }
            s if s.starts_with("hsv(") => {
                let hsv = parser::hsv::parse_hsv_str(s)?;
                let (r, g, b) = conversion::hsv::hsv2rgb(hsv);
                (r, g, b, 1.0)
            }
            s if s.starts_with("hsi(") => {
                let hsi = parser::hsi::parse_hsi_str(s)?;
                let (r, g, b) = conversion::hsi::hsi2rgb(hsi);
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
            s if s.starts_with("yuv(") => {
                let yuv = parser::yuv::parse_yuv_str(s)?;
                let (r, g, b) = conversion::yuv::yuv2rgb(yuv);
                (r, g, b, 1.0)
            }
            s if s.starts_with("ycbcr(") => {
                let ycbcr = parser::ycbcr::parse_ycbcr_str(s)?;
                let (r, g, b) = conversion::ycbcr::ycbcr2rgb(ycbcr);
                (r, g, b, 1.0)
            }
            s if s.starts_with("lab(") => {
                let lab = parser::lab::parse_lab_str(s)?;
                let (r, g, b) = conversion::lab::lab2rgb(lab);
                (r, g, b, 1.0)
            }
            _ => {
                let found = hex_of_name(s);
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
    fn test_color_from_hsla_str() {
        let s = "hsla(180, 100%, 50%, 0.6)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color.hsla(), "hsla(180, 100%, 50%, 0.6)");
        assert_eq!(color.rgba(), "rgba(0, 255, 255, 0.6)");
    }

    #[test]
    fn test_color_from_hsv_str() {
        let s = "hsv(180, 100%, 100%)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color.hsv(), "hsv(180, 100%, 100%)");
        assert_eq!(color.rgb(), "rgb(0, 255, 255)");
    }

    #[test]
    fn test_color_from_hsi_str() {
        let s = "hsi(240°, 100%, 33.33%)";
        let color = Color::from_str(s).unwrap();
        assert_eq!(color.hsi(), "hsi(240, 100%, 33.33%)");
        assert_eq!(color.rgb(), "rgb(0, 0, 255)");
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
        assert_eq!(color.hex(), "#f00");

        let color = Color::from_str("cmyk(100%, 0, 100%, 0)").unwrap();
        assert_eq!(color.hex(), "#0f0");

        let color = Color::from_str("cmyk(100%, 100%, 0, 0)").unwrap();
        assert_eq!(color.hex(), "#00f");

        let color = Color::from_str("cmyk(0, 0, 0, 100%)").unwrap();
        assert_eq!(color.hex(), "#000");

        let color = Color::from_str("cmyk(0, 0, 0, 0)").unwrap();
        assert_eq!(color.hex(), "#fff");

        let color = Color::from_str("cmyk(20%, 80%, 0, 0)").unwrap();
        assert_eq!(color.hex(), "#c3f");

        let color = Color::from_str("cmyk(35%, 0, 60%, 0)").unwrap();
        assert_eq!(color.hex(), "#a6ff66");
    }

    #[test]
    fn test_color_from_xyz_str() {
        let color = Color::from_str("xyz(0.412453, 0.212671, 0.019334)").unwrap();
        assert_eq!(color.rgb(), "rgb(255, 0, 0)");

        let color = Color::from_str("xyz(0.70047, 0.723315, 1.048516)").unwrap();
        assert_eq!(color.rgb(), "rgb(92, 122, 255)");
    }

    #[test]
    fn test_color_from_yuv_str() {
        let color = Color::from_str("yuv(0.2126, 0.4240, 0.0593)").unwrap();
        assert_eq!(color.rgb(), "rgb(71, 3, 255)");

        let color = Color::from_str("yuv(0.7233, -0.2747, -0.3212)").unwrap();
        assert_eq!(color.rgb(), "rgb(91, 255, 42)");
    }

    #[test]
    fn test_color_from_ycbcr_str() {
        let color = Color::from_str("YCbCr(225.93, 0.5755, 148.7269)").unwrap();
        assert_eq!(color.rgb(), "rgb(255, 255, 0)");
    }
}
