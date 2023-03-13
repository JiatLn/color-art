use crate::{parser::*, Color};
use anyhow::{Ok, Result};

/// Color space enum.
#[derive(Clone)]
pub enum ColorSpace {
    /// RGB color space.
    ///
    /// RGB stands for red, green, and blue.
    RGB,
    /// RGBA color space.
    ///
    /// RGBA stands for red, green, blue, and alpha.
    RGBA,
    /// HSL color space.
    ///
    /// HSL stands for hue, saturation, and lightness.
    HSL,
    /// HSV color space.
    ///
    /// HSV stands for hue, saturation, and value.
    HSV,
    /// Hex color space.
    ///
    /// Hex stands for hexadecimal.
    HEX,
    /// HWB color space.
    ///
    /// HWB stands for hue, whiteness, and blackness.
    HWB,
    /// CMYK color space.
    ///
    /// CMYK means Cyan Magenta Yellow Black
    CMYK,
    /// XYZ color space.
    XYZ,
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
        match s.to_string().as_str() {
            "rgb" => ColorSpace::RGB,
            "rgba" => ColorSpace::RGBA,
            "hsl" => ColorSpace::HSL,
            "hsv" => ColorSpace::HSV,
            "hex" => ColorSpace::HEX,
            "hwb" => ColorSpace::HWB,
            "cmyk" => ColorSpace::CMYK,
            "xyz" => ColorSpace::XYZ,
            _ => ColorSpace::Unknown,
        }
    }
}

impl Color {
    /// Get the color space vector of the color instance.
    pub fn space(&self, space: ColorSpace) -> Result<Vec<f64>> {
        match space {
            ColorSpace::RGB => {
                let (r, g, b) = self.rgb;
                Ok(vec![r, g, b])
            }
            ColorSpace::RGBA => {
                let (r, g, b) = self.rgb;
                Ok(vec![r, g, b, self.alpha])
            }
            ColorSpace::HSL => {
                let (h, s, l) = hsl::parse_hsl_str(self.hsl())?;
                Ok(vec![h, s, l])
            }
            ColorSpace::HSV => {
                let (h, s, v) = hsv::parse_hsv_str(self.hsv())?;
                Ok(vec![h, s, v])
            }
            ColorSpace::HWB => {
                let (h, w, b) = hwb::parse_hwb_str(self.hwb())?;
                Ok(vec![h, w, b])
            }
            ColorSpace::CMYK => {
                let (c, m, y, k) = cmyk::parse_cmyk_str(self.cmyk())?;
                Ok(vec![c, m, y, k])
            }
            ColorSpace::XYZ => {
                let (x, y, z) = xyz::parse_xyz_str(self.xyz())?;
                Ok(vec![x, y, z])
            }
            ColorSpace::HEX | ColorSpace::Unknown => {
                anyhow::bail!("not implemented yet")
            }
        }
    }
}
