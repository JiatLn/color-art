use crate::{parser::*, Color};
use anyhow::{Ok, Result};

/// Color space enum.
pub enum ColorSpace {
    /// RGB color space.
    ///
    /// RGB stands for red, green, and blue.
    Rgb,
    /// RGBA color space.
    ///
    /// RGBA stands for red, green, blue, and alpha.
    Rgba,
    /// HSL color space.
    ///
    /// HSL stands for hue, saturation, and lightness.
    Hsl,
    /// HSV color space.
    ///
    /// HSV stands for hue, saturation, and value.
    Hsv,
    /// Hex color space.
    ///
    /// Hex stands for hexadecimal.
    Hex,
    /// Unknown color space.
    ///
    /// To be used when the color space is not known.
    Unknown,
}

impl<T> From<T> for ColorSpace
where
    T: ToString,
{
    fn from(s: T) -> Self {
        let s = s.to_string();
        match s.as_str() {
            "rgb" => ColorSpace::Rgb,
            "rgba" => ColorSpace::Rgba,
            "hsl" => ColorSpace::Hsl,
            "hsv" => ColorSpace::Hsv,
            "hex" => ColorSpace::Hex,
            _ => ColorSpace::Unknown,
        }
    }
}

impl ColorSpace {
    pub fn valid(&self, vec: Vec<f64>) -> Result<()> {
        match self {
            ColorSpace::Rgb => todo!(),
            ColorSpace::Rgba => todo!(),
            ColorSpace::Hsl => {
                if vec.len() != 3 {
                    anyhow::bail!("HSL color space requires 3 values")
                }
                let h = vec[0];
                let s = vec[1];
                let l = vec[2];
                if h < 0.0 || h > 360.0 {
                    anyhow::bail!("Hue must be between 0.0 and 360.0, got {}", h)
                }
                if s < 0.0 || s > 1.0 {
                    anyhow::bail!("Saturation must be between 0.0 and 1.0, got {}", s)
                }
                if l < 0.0 || l > 1.0 {
                    anyhow::bail!("Lightness must be between 0.0 and 1.0, got {}", l)
                }
                Ok(())
            }
            ColorSpace::Hsv => todo!(),
            ColorSpace::Hex => todo!(),
            ColorSpace::Unknown => todo!(),
        }
    }
}

impl Color {
    /// Get the color space vector of the color instance.
    pub fn space(&self, space: ColorSpace) -> Result<Vec<f64>> {
        match space {
            ColorSpace::Rgb => {
                let (r, g, b, _a) = self.rgba;
                Ok(vec![r, g, b])
            }
            ColorSpace::Rgba => {
                let (r, g, b, a) = self.rgba;
                Ok(vec![r, g, b, a])
            }
            ColorSpace::Hsl => {
                let (h, s, l) = hsl::parse_hsl_str(self.hsl())?;
                Ok(vec![h, s, l])
            }
            ColorSpace::Hsv => {
                let (h, s, v) = hsv::parse_hsv_str(self.hsv())?;
                Ok(vec![h, s, v])
            }
            ColorSpace::Hex | ColorSpace::Unknown => {
                anyhow::bail!("not implemented yet")
            }
        }
    }
}
