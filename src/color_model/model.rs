use crate::{parser::*, Color};
use anyhow::{Ok, Result};

pub enum ColorModel {
    /// RGB color model.
    ///
    /// RGB stands for red, green, and blue.
    Rgb,
    /// RGBA color model.
    ///
    /// RGBA stands for red, green, blue, and alpha.
    Rgba,
    /// HSL color model.
    ///
    /// HSL stands for hue, saturation, and lightness.
    Hsl,
    /// HSV color model.
    ///
    /// HSV stands for hue, saturation, and value.
    Hsv,
    /// Hex color model.
    ///
    /// Hex stands for hexadecimal.
    Hex,
    /// Unknown color model.
    ///
    /// To be used when the color model is not known.
    Unknown,
}

impl<T> From<T> for ColorModel
where
    T: ToString,
{
    fn from(s: T) -> Self {
        let s = s.to_string();
        match s.as_str() {
            "rgb" => ColorModel::Rgb,
            "rgba" => ColorModel::Rgba,
            "hsl" => ColorModel::Hsl,
            "hsv" => ColorModel::Hsv,
            "hex" => ColorModel::Hex,
            _ => ColorModel::Unknown,
        }
    }
}

impl Color {
    /// Get the color model vector of the color instance.
    pub fn model(&self, model: ColorModel) -> Result<Vec<f64>> {
        match model {
            ColorModel::Rgb => {
                let (r, g, b, _a) = self.rgba;
                Ok(vec![r, g, b])
            }
            ColorModel::Rgba => {
                let (r, g, b, a) = self.rgba;
                Ok(vec![r, g, b, a])
            }
            ColorModel::Hsl => {
                let (h, s, l) = hsl::parse_hsl_str(self.hsl())?;
                Ok(vec![h, s, l])
            }
            ColorModel::Hsv => {
                let (h, s, v) = hsv::parse_hsv_str(self.hsv())?;
                Ok(vec![h, s, v])
            }
            ColorModel::Hex | ColorModel::Unknown => {
                anyhow::bail!("not implemented yet")
            }
        }
    }
}
