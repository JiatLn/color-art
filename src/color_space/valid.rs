use crate::ColorSpace;
use anyhow::Result;

impl ColorSpace {
    /// Check if a vector of values is valid for a given color space.
    pub fn valid(&self, vec: Vec<f64>) -> Result<()> {
        match self {
            ColorSpace::RGB => todo!(),
            ColorSpace::RGBA => todo!(),
            ColorSpace::HSL => {
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
            ColorSpace::HSV => todo!(),
            ColorSpace::HEX => todo!(),
            ColorSpace::Unknown => todo!(),
        }
    }
}
